use semio_framework_pack_record_retained::RetainedRecordBodyCursor;
use semio_framework_replication_codec::PackLimits;
use semio_framework_value::{RetainedAllocationEffect, ValueError, retained_clone::RetainedCloneGrant};
use semio_framework_value_control_contract::{OriginalWork, OriginalWorkAuthority, OriginalWorkPartialCheck, OriginalWorkPartialCheckComplete, OriginalWorkQuote, OriginalWorkQuoteComplete, OriginalWorkRecipient, OriginalSourceTurnIdentity};

struct Payer;
impl OriginalWorkRecipient for Payer {
    type Context<'a,'b> = () where 'b:'a;
    fn validate_context<'a,'b>(&self, _: &()) -> Result<(), ValueError> where 'b:'a { Ok(()) }
    fn source_turn<'a,'b>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'b:'a { None }
    fn admits<'r,'a,'b>(&mut self, _: &mut (), quote: OriginalWorkQuote<'r>, _: u64, _: u64) -> OriginalWorkQuoteComplete<'r> where 'b:'a { quote.complete(Ok(true)) }
    fn settle_partial<'r,'a,'b>(&mut self, _: &mut (), check: OriginalWorkPartialCheck<'r>) -> OriginalWorkPartialCheckComplete<'r> where 'b:'a { check.complete(Ok(())) }
    fn recover_partial<'r,'a,'b>(&mut self, _: &mut (), check: OriginalWorkPartialCheck<'r>) -> OriginalWorkPartialCheckComplete<'r> where 'b:'a { check.pending() }
}

fn run_effect(grant: RetainedCloneGrant, maximum: u64, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> bool) -> bool {
    let mut authority = OriginalWorkAuthority::new(Payer);
    authority.with_loan(&mut (), |loan| {
        let mut loan = loan.reborrow();
        let mut work = OriginalWork::empty();
        work.recipient = Some(&mut loan);
        let (mut action, mut receipt, mut proof, mut marker) = (None, None, None, None);
        work.prepare_for_action(1, &mut action).unwrap();
        work.issue_performed_for_action(action, &mut receipt).unwrap();
        let mut effect = RetainedAllocationEffect::new(grant, maximum, &mut marker, receipt.as_mut().unwrap());
        let result = run(&mut effect);
        let spent = effect.performed_work();
        work.claim_performed_for_action(action, &mut receipt).unwrap();
        if spent == 0 {
            work.cancel_for_action(&mut action).unwrap();
        } else {
            let mut partial = None;
            work.settle_partial_for_action_into(action, &mut partial).unwrap();
            work.commit_for_action(spent, action, &mut proof).unwrap();
            work.finish_for_action(&mut proof, &mut action).unwrap();
        }
        result
    }).unwrap()
}

#[no_mangle]
pub extern "C" fn check() -> u32 {
    let limits = PackLimits { max_file_len: 64, max_segment_len: 64, max_symbols: 1, max_depth: 8, max_items: 64, max_total_alloc: 131072 };
    let mut cursor = RetainedRecordBodyCursor::try_new(limits, 0, 0, 0, 131072).unwrap();
    cursor.admit_byte(0, 0).unwrap();
    let required = loop {
        if let Some(bytes) = cursor.next_allocation_bytes().unwrap() { break bytes; }
        cursor.grant().unwrap();
    };
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 8192, maximum_capacity_bytes: required, maximum_release_bytes: required, maximum_depth: 64 };
    let denied = run_effect(grant, 0, |effect| {
        let result = cursor.admit_record_storage(effect).unwrap();
        result.retained_capacity_bytes == 0 && effect.actual().is_none() && effect.performed_work() == 0
    });
    let accepted = run_effect(grant, 1, |effect| {
        let result = cursor.admit_record_storage(effect).unwrap();
        result.retained_capacity_bytes == required && *effect.actual() == Some(required) && effect.performed_work() == 1
    });
    for _ in 0..128 {
        if cursor.terminal_is_empty() { return u32::from(denied) + 2*u32::from(accepted); }
        run_effect(grant, 1, |effect| cursor.retire_record_storage(8192, effect).is_ok());
    }
    0
}
