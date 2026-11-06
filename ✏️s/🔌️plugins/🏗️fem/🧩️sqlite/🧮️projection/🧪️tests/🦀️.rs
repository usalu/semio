//! 🧪️ Borrowed forecast actions remain distinct from physical row receipts.
use super::*;
use semio_framework_io_sqlite_snapshot::SqliteDatabaseLimits;
#[test]
fn controlled_forecast_counts_selected_row_and_field_actions(){
 let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits::default());let mut actions=0;
 {let mut action=||{actions+=1;Ok(())};let mut writer=RowWriter::forecast_controlled(&mut control,SqliteSnapshotPhase::DecodeNative,&mut action).unwrap();writer.work().unwrap();writer.insert_float("node",&[Cell::Text("a"),Cell::Real(1.)],&[FloatColumn::Binary64(2)]).unwrap();writer.finish_forecast(1).unwrap();}
 assert_eq!(actions,5);
}
#[test]
fn controlled_forecast_stops_before_the_denied_field(){
 let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits::default());let(mut attempted,mut performed)=(0,0);
 let error={let mut action=||{attempted+=1;if attempted==3{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"denied original action"))}performed+=1;Ok(())};let mut writer=RowWriter::forecast_controlled(&mut control,SqliteSnapshotPhase::DecodeNative,&mut action).unwrap();writer.work().unwrap();writer.insert_float("node",&[Cell::Text("a"),Cell::Real(1.)],&[FloatColumn::Binary64(2)]).unwrap_err()};
 assert_eq!(error.kind,ValueRefusalKind::WorkLimit);assert_eq!(attempted,3);assert_eq!(performed,2);
}

/// 🎟️ This finite lower-level loan exercises the actual native control, independently of job caller tests.
struct DecodeLoan(u64);
impl semio_framework_value::native_decoding::NativeDecodeWorkRecipient for DecodeLoan{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|value|value<=self.0))}}
struct EncodeLoan{limit:u64,posted:u64}
impl semio_framework_value::native_encoding::NativeEncodeWorkRecipient for EncodeLoan{
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|value|value<=self.limit))}
 fn committed(&mut self,actual:u64)->Result<(),ValueError>{self.posted=self.posted.checked_add(actual).filter(|value|*value<=self.limit).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"finite forecast encoding loan"))?;Ok(())}
}
#[test]
fn forecast_debits_the_original_decode_and_encode_actions(){
 for limit in [2u64,5]{
  let mut observer=|_|true;let mut native=semio_framework_value::NativeDecodeControl::new(0,&mut observer);let mut loan=DecodeLoan(limit);let mut selected=0u64;
  let result=native.with_work_recipient(&mut loan,|native| {
   let mut sql_observer=|_|true;let mut sql=SqliteSnapshotControl::new(&mut sql_observer,SqliteDatabaseLimits::default());
   let mut action=||{native.checkpoint()?;if !native.prepare_work(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"forecast decoding denied"))}let result=native.commit_work(1);selected+=1;result};
   let mut writer=RowWriter::forecast_controlled(&mut sql,SqliteSnapshotPhase::DecodeNative,&mut action)?;writer.work()?;writer.insert_float("node",&[Cell::Text("a"),Cell::Real(1.)],&[FloatColumn::Binary64(2)])?;writer.finish_forecast(1)
  });
  assert_eq!(selected,limit);assert_eq!(native.actual_work(),selected);assert_eq!(result.is_ok(),limit==5);if let Err(error)=result{assert_eq!(error.kind,ValueRefusalKind::WorkLimit)}
  let mut observer=|_|true;let mut native=semio_framework_value::NativeEncodeControl::new(0,&mut observer);let mut loan=EncodeLoan{limit,posted:0};let mut selected=0u64;
  let result=native.with_work_recipient(&mut loan,|native| {
   let mut sql_observer=|_|true;let mut sql=SqliteSnapshotControl::new(&mut sql_observer,SqliteDatabaseLimits::default());
   let mut action=||{native.checkpoint()?;if !native.prepare_work(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"forecast encoding denied"))}let result=native.commit_work(1);selected+=1;result};
   let mut writer=RowWriter::forecast_controlled(&mut sql,SqliteSnapshotPhase::EncodeNative,&mut action)?;writer.work()?;writer.insert_float("node",&[Cell::Text("a"),Cell::Real(1.)],&[FloatColumn::Binary64(2)])?;writer.finish_forecast(1)
  });
  assert_eq!(selected,limit);assert_eq!(native.actual_work(),selected);assert_eq!(loan.posted,selected);assert_eq!(result.is_ok(),limit==5);if let Err(error)=result{assert_eq!(error.kind,ValueRefusalKind::WorkLimit)}
 }
}

#[test]
fn census_keeps_table_counts_and_refuses_before_an_unfunded_increment(){
 let names=["a","b"];let mut counts=[0usize;2];let mut work=0;
 {let mut action=||{work+=1;Ok(())};let mut census=TableCensus::new(&names,&mut counts,&mut action).unwrap();census.work().unwrap();assert_eq!(census.insert("a",&[]).unwrap(),1);assert_eq!(census.insert("b",&[]).unwrap(),1);census.insert_key_float("b",7,&[],&[]).unwrap();}
 assert_eq!(counts,[1,2]);assert_eq!(work,11);
 let mut counts=[0usize;2];let mut selected=0;let error={let mut action=||{if selected==4{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"denied census increment"))}selected+=1;Ok(())};let mut census=TableCensus::new(&names,&mut counts,&mut action).unwrap();census.insert("b",&[]).unwrap_err()};
 assert_eq!(error.kind,ValueRefusalKind::WorkLimit);assert_eq!(selected,4);assert_eq!(counts,[0,0]);
}

#[cfg(target_pointer_width="64")]
#[test]
fn census_identity_overflow_preserves_the_existing_count(){
 let names=["a"];let mut counts=[i64::MAX as usize];let mut action=||Ok(());
 let error={let mut census=TableCensus{names:&names,counts:&mut counts,action:&mut action};census.insert("a",&[]).unwrap_err()};
 assert_eq!(error.kind,ValueRefusalKind::WorkLimit);assert_eq!(counts,[i64::MAX as usize]);
}


#[test]
fn duplicate_ieee_column_comparisons_are_admitted(){
 let mut observer=|_|true;let mut control=SqliteSnapshotControl::new(&mut observer,SqliteDatabaseLimits::default());let mut selected=0;
 let error={let mut action=||{selected+=1;Ok(())};let mut writer=RowWriter::forecast_controlled(&mut control,SqliteSnapshotPhase::DecodeNative,&mut action).unwrap();writer.insert_float("node",&[Cell::Real(1.)],&[FloatColumn::Binary64(1),FloatColumn::Binary64(1)]).unwrap_err()};
 assert_eq!(error.kind,ValueRefusalKind::InvalidValue);assert_eq!(selected,5);
}

#[test]
fn field_read_waits_for_its_work_admission(){
 let mut observer=|_|true;let mut control=SqliteSnapshotControl::new(&mut observer,SqliteDatabaseLimits::default());let mut reads=0;
 let result={let mut action=||Err(ValueError::literal(ValueRefusalKind::WorkLimit,"denied field read"));let mut writer=RowWriter::forecast_controlled(&mut control,SqliteSnapshotPhase::DecodeNative,&mut action).unwrap();(||->Result<(),ValueError>{let cells=[{writer.work()?;reads+=1;Cell::Integer(1)}];writer.insert("node",&cells)?;Ok(())})()};
 assert_eq!(result.unwrap_err().kind,ValueRefusalKind::WorkLimit);assert_eq!(reads,0);
}
