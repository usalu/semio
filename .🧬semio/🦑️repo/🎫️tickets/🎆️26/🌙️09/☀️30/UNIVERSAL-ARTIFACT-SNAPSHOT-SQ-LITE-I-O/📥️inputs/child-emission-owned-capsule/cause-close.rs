fn protocol_owned_cause_text(cause:&mut ::protocol::ProtocolError)->Result<Option<&mut String>,()> {
    use ::protocol::ProtocolError as Error;
    match cause{
        Error::Malformed{detail,..}|Error::Io(detail)=>Ok((detail.capacity()!=0).then_some(detail)),
        Error::Pack(error)=>match error{
            store::PackError::TransportFailure(_)=>Err(()),
            store::PackError::Refusal(refusal)=>match refusal{
                store::PackRefusal::Malformed{detail,..}=>Ok((detail.capacity()!=0).then_some(detail)),
                store::PackRefusal::ValueRefusal(error)|store::PackRefusal::Io{error,..}=>Ok((error.message.capacity()!=0).then_some(&mut error.message)),
                store::PackRefusal::TextRefusal(error)=>{
                    if error.message.capacity()!=0{return Ok(Some(&mut error.message));}
                    Ok(error.expected.as_mut().filter(|value|value.capacity()!=0))
                },
                store::PackRefusal::BadMagic|store::PackRefusal::UnsupportedVersion{..}|store::PackRefusal::UnknownRequiredFlags(_)|store::PackRefusal::Truncated(_)|store::PackRefusal::ChecksumMismatch{..}|store::PackRefusal::ContentHashMismatch|store::PackRefusal::LimitExceeded{..}|store::PackRefusal::RetainedMalformed{..}|store::PackRefusal::RetainedAllocation{..}|store::PackRefusal::NonCanonical(_)|store::PackRefusal::UnsupportedCodec(_)|store::PackRefusal::TransportAdmission{..}=>Ok(None),
            }
        },
        Error::ChainMismatch{..}|Error::TornTail(_)|Error::UnknownCriticalRecord(_)|Error::DictMiss(_)|Error::DictOutOfOrder{..}|Error::VerifierRequired|Error::SignatureInvalid{..}|Error::FrameFraming(_)|Error::LimitExceeded(_)=>Ok(None),
    }
}
fn close_protocol_owned_cause_one(cause:&mut Option<::protocol::ProtocolError>,maximum_bytes:usize)->PluginCloseStep{
    let Some(error)=cause.as_mut()else{return PluginCloseStep::Complete};
    match protocol_owned_cause_text(error){
        Err(())=>PluginCloseStep::AwaitingInput{reason:"owned encoder transport cause requires its genuine provider retirement handoff"},
        Ok(Some(text))=>{
            let bytes=text.capacity();
            if bytes>maximum_bytes{return PluginCloseStep::Pending{released_items:0,released_bytes:0};}
            *text=String::new();
            PluginCloseStep::Pending{released_items:1,released_bytes:bytes}
        },
        Ok(None)=>{cause.take();PluginCloseStep::Pending{released_items:1,released_bytes:0}},
    }
}
