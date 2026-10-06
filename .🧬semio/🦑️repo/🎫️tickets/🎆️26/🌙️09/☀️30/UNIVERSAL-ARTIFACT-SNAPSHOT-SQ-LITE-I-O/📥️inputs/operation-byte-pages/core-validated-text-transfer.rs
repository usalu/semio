/// 🧾️ Transfers octets after controlled validation of the same immutable source range.
/// Both private admission owners use the exact span copy; the immutable borrow prevents its
/// validated scalar words from changing between validation and owned transfer.
fn copy_validated_span_utf8<M:MaterializationAdmission+?Sized>(materialization:&M,bytes:crate::ByteSpan<'_>,offset:u64,what:&'static str)->Result<String,PackRefusal>{
    materialization.check_span_utf8(bytes,offset,what)?;
    let bytes=materialization.copy_span_bytes(bytes)?;
    Ok(unsafe{String::from_utf8_unchecked(bytes)})
}
