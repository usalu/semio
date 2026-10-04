#[semio_framework_async_macros::async_test]
async fn pack_refusal_kind_controls_db_category_and_owned_message() {
    use semio_framework_value::{ValueError,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚠️error/🎒️pack/🔣️.json")).unwrap();
    fn kind(value:&str)->ValueRefusalKind{match value{"InvalidValue"=>ValueRefusalKind::InvalidValue,"Canceled"=>ValueRefusalKind::Canceled,"OwnershipLimit"=>ValueRefusalKind::OwnershipLimit,"AllocationFailed"=>ValueRefusalKind::AllocationFailed,"WorkLimit"=>ValueRefusalKind::WorkLimit,"DepthLimit"=>ValueRefusalKind::DepthLimit,"UnsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"InvariantViolated"=>ValueRefusalKind::InvariantViolated,_=>panic!("unknown authored kind")}}
    for row in fixture["cases"].as_array().unwrap(){
        let mut retained=None;
        let error=match row["source"].as_str().unwrap(){
            "ValueRefusal"=>{let mut cause=ValueError::new(kind(row["kind"].as_str().unwrap()),row["message"].as_str().unwrap());for part in row["path"].as_array().unwrap(){cause=cause.under(part.as_str().unwrap());}if cause.kind==ValueRefusalKind::InvalidValue{retained=Some(cause.message.as_ptr());}pack::PackError::ValueRefusal(cause)},
            "Io"=>pack::PackError::Io(row["message"].as_str().unwrap().to_owned()),
            "LimitExceeded"=>pack::PackError::LimitExceeded("too big"),
            "BadMagic"=>pack::PackError::BadMagic,
            "Truncated"=>pack::PackError::Truncated(row["offset"].as_u64().unwrap()),
            _=>panic!("unknown authored source")
        };
        let converted=DbError::from(error);
        let(category,message)=match &converted{DbError::InvalidArgument(message)=>("InvalidArgument",message.as_str()),DbError::Corrupt(message)=>("Corrupt",message.as_str()),DbError::Io(message)=>("Io",message.as_str()),DbError::LimitExceeded(message)=>("LimitExceeded",*message),_=>panic!("unexpected conversion category")};
        assert_eq!(serde_json::json!({"category":category,"output":message}),serde_json::json!({"category":row["category"],"output":row["output"]}),"{}",row["id"]);
        if let Some(pointer)=retained{let DbError::InvalidArgument(message)=converted else{panic!("typed invalid argument")};assert_eq!(message.as_ptr(),pointer);}
    }
    eprintln!("[DEBUG] DB Pack conversion twelve closed vectors use typed kind and retain invalid argument allocation");
}
