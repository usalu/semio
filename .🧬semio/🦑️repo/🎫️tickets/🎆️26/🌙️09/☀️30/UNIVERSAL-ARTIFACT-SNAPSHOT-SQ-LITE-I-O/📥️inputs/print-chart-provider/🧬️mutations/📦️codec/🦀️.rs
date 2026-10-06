//! 🎮️ Canonical typed chart operations share the authored Record fields in Text and binary.
use crate::ChangeChartValue;
impl protocol::OpText for ChangeChartValue{
 fn print_op(&self)->String{semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Inline)}
 fn parse_op(line:&str)->Result<Self,semio_framework_diagnostic::TextError>{Self::__dsl_from_record(&semio_framework_dsl_record::parse_exact(line,&Self::__dsl_spec(),&Default::default())?)}
}
impl protocol::OpBinary for ChangeChartValue{
 fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{let body=pack::record::encode_record_body(&Self::__dsl_spec(),&self.__dsl_to_record(),&Default::default())?;let mut bytes=vec![1,1];bytes.extend(body);Ok(bytes)}
 fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{if !bytes.starts_with(&[1,1]){return Err(protocol::ProtocolError::from(protocol::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"Chart operation format/tag mismatch"))))}let record=pack::record::decode_record_body_exact(&bytes[2..],&Self::__dsl_spec(),&Default::default())?;Self::__dsl_from_record(&record).map_err(|error|protocol::ProtocolError::from(protocol::PackError::from(error)))}
}
