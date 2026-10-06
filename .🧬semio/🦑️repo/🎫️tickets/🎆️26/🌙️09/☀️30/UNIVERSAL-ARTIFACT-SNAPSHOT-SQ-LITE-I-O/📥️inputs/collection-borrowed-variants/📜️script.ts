import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {resolve,join} from "node:path";

const repo=resolve(import.meta.dir,"../../../../../../../../..");
const owner="🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection";
const methods=String.raw`
    fn projected_variant_identity(&self)->(&'static str,usize,semio_framework_dsl_record::RecordSpecProducer){
        match self{Self::Document{..}=>("document",0,artifact_body_document_producer()),Self::Blob{..}=>("blob",1,artifact_body_blob_producer())}
    }
    fn projected_variant_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,semio_framework_value::ValueError>{
        use semio_framework_dsl_record::{DslField,native_encoding::{FieldProjectionView as V,projection_path_error}};
        if path.is_empty(){return Ok(V::Record(match self{Self::Document{..}=>&[0,1],Self::Blob{..}=>&[0,1,2]}))}
        let tail=&path[1..];
        match(self,path[0]){
            (Self::Document{schema,..},0)=>DslField::projection_view(schema,tail),
            (Self::Document{document_id,..},1)=>DslField::projection_view(document_id,tail),
            (Self::Blob{blob},0)=>DslField::projection_view(&blob.hash,tail),
            (Self::Blob{blob},1)=>DslField::projection_view(&blob.size,tail),
            (Self::Blob{blob},2)=>DslField::projection_view(&blob.media_type,tail),
            _=>Err(projection_path_error())
        }
    }
    fn projected_variant_key(&self,path:&[usize],index:usize)->Result<&str,semio_framework_value::ValueError>{
        use semio_framework_dsl_record::{DslField,native_encoding::projection_path_error};
        if path.is_empty(){return Err(projection_path_error())}
        let tail=&path[1..];
        match(self,path[0]){
            (Self::Document{schema,..},0)=>DslField::projection_key(schema,tail,index),
            (Self::Document{document_id,..},1)=>DslField::projection_key(document_id,tail,index),
            (Self::Blob{blob},0)=>DslField::projection_key(&blob.hash,tail,index),
            (Self::Blob{blob},1)=>DslField::projection_key(&blob.size,tail,index),
            (Self::Blob{blob},2)=>DslField::projection_key(&blob.media_type,tail,index),
            _=>Err(projection_path_error())
        }
    }
`;
const demand=String.raw`

#[test]
fn sqlite_snapshot_framework_collection_original_borrowed_variants_match_serde_owner(){
    use semio_framework_dsl_record::{DslVariants,native_encoding::FieldProjectionView as V};
    let schema:serde_json::Value=serde_json::from_str(include_str!("../../../../\u{1f9eb}\u{fe0f}fixtures/\u{1f3ed}\u{fe0f}native-schema/\u{1f523}\u{fe0f}.json")).unwrap();
    let neutral=f();let source=fixture();let mut sizes=neutral["blobSizes"].as_array().unwrap().iter();let mut witnessed=std::collections::BTreeSet::new();
    let authored_entries=neutral["nativeSnapshot"]["entries"].as_array().unwrap();assert_eq!(source.entries.len(),authored_entries.len());
    for(entry,authored)in source.entries.iter().zip(authored_entries){
        let body=entry.body.as_ref();let(keyword,ordinal,producer)=body.projected_variant_identity();witnessed.insert(keyword);
        assert_eq!(schema["variants"][ordinal]["keyword"],keyword);let spec=(producer.ordinary)();let mut expected=authored["body"].clone();
        if keyword=="blob"{expected["blob"]["size"]=serde_json::json!(sizes.next().unwrap().as_str().unwrap().parse::<u64>().unwrap());}
        let third_party=serde_json::to_value(body).unwrap();assert_eq!(third_party,expected);
        let ids=match body.projected_variant_view(&[]).unwrap(){V::Record(ids)=>ids,_=>panic!("borrowed variant is an original Record")};
        assert_eq!(ids,spec.fields.iter().map(|field|field.id).collect::<Vec<_>>());
        let mut projected=Vec::new();
        for index in 0..ids.len(){match body.projected_variant_view(&[index]).unwrap(){
            V::Text(text)=>{let original=match(body,index){(ArtifactBody::Document{schema,..},0)=>schema,(ArtifactBody::Document{document_id,..},1)=>document_id,(ArtifactBody::Blob{blob},0)=>&blob.hash,(ArtifactBody::Blob{blob},2)=>&blob.media_type,_=>panic!("declared borrowed text ordinal")};assert_eq!((text.as_ptr(),text.len()),(original.as_ptr(),original.len()));projected.push(serde_json::json!(text));},
            V::UInt(value)=>projected.push(serde_json::json!(value)),_=>panic!("complete authored scalar variant")
        }}
        let independently_serialized=match keyword{"document"=>serde_json::json!([third_party["schema"],third_party["document_id"]]),"blob"=>serde_json::json!([third_party["blob"]["hash"],third_party["blob"]["size"],third_party["blob"]["mediaType"]]),_=>panic!("closed authored variant")};
        assert_eq!(serde_json::Value::Array(projected),independently_serialized);assert!(body.projected_variant_view(&[ids.len()]).is_err());assert!(body.projected_variant_view(&[0,0]).is_err());assert!(body.projected_variant_key(&[],0).is_err());assert!(body.projected_variant_key(&[0],0).is_err());
    }
    assert_eq!(witnessed,std::collections::BTreeSet::from(["document","blob"]));assert!(sizes.next().is_none());
    eprintln!("[DEBUG] Collection original borrowed ArtifactBody variants=2 exact_scalar_fields=true original_text_pointers=true serde_full_owner=true");
}
`;
const pairPath=join(import.meta.dir,"guarded-pairs.json");
if(process.argv[2]==="stage"){
    const sourcePath=join(owner,"🦀️.rs"),testPath=join(owner,"🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs");
    const before=readFileSync(join(repo,sourcePath),"utf8"),testBefore=readFileSync(join(repo,testPath),"utf8");
    const anchor="impl semio_framework_dsl_record::DslVariants for ArtifactBody {";
    if(before.includes("fn projected_variant_identity")||!before.includes(anchor)||testBefore.includes("fn sqlite_snapshot_framework_collection_original_borrowed_variants_match_serde_owner"))throw Error("new exact owner demand guard");
    mkdirSync(import.meta.dir,{recursive:true});writeFileSync(pairPath,JSON.stringify({productionMutations:0,pairs:[{path:sourcePath,before,after:before.replace(anchor,anchor+methods)},{path:testPath,before:testBefore,after:testBefore+demand}]},null,2)+"\n");
    console.log("[DEBUG] Collection exact borrowed variant held paths=2 production_mutations=0");
}else if(process.argv[2]==="mount"){
    const capsule=JSON.parse(readFileSync(pairPath,"utf8"));for(const pair of capsule.pairs){if(readFileSync(join(repo,pair.path),"utf8")!==pair.before)throw Error("concurrent exact guard changed "+pair.path);}
    for(const pair of capsule.pairs)writeFileSync(join(repo,pair.path),pair.after);
    console.log("[DEBUG] Collection original borrowed variant mounted paths="+capsule.pairs.length);
}else if(process.argv[2]==="spelling"){
    const path=join(owner,"🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs"),before=readFileSync(join(repo,path),"utf8");
    const anchor=String.raw`    for(entry,authored)in source.entries.iter().zip(neutral["nativeSnapshot"]["entries"].as_array().unwrap()){`;
    if(!before.includes(anchor)||!before.includes(String.raw`\uFE0F`))throw Error("exact fixture spelling/cardinality guard");
    const after=before.replaceAll(String.raw`\uFE0F`,String.raw`\u{fe0f}`).replace(anchor,String.raw`    let authored_entries=neutral["nativeSnapshot"]["entries"].as_array().unwrap();assert_eq!(source.entries.len(),authored_entries.len());
    for(entry,authored)in source.entries.iter().zip(authored_entries){`);
    writeFileSync(join(import.meta.dir,"fixture-spelling-cardinality-pair.json"),JSON.stringify({pairs:[{path,before,after}]},null,2)+"\n");
    if(readFileSync(join(repo,path),"utf8")!==before)throw Error("concurrent fixture spelling guard changed");writeFileSync(join(repo,path),after);
    console.log("[DEBUG] Collection exact Rust fixture spelling and full owner cardinality mounted paths=1");
}else if(process.argv[2]==="ledger-authority"){
    const testPath=join(owner,"🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs"),fixturePath=join(owner,"🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json"),hostPath="🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs";
    const testBefore=readFileSync(join(repo,testPath),"utf8"),fixtureBefore=readFileSync(join(repo,fixturePath),"utf8"),hostBefore=readFileSync(join(repo,hostPath),"utf8");
    const oldCharge="let charged=limits.max_value_bytes-control.reconstruction_remaining_bytes().unwrap();",oldCeiling="let limits=SqliteDatabaseLimits{max_value_bytes:ceiling,..limits};",ratio='let ratio=&corpus["nativeRetention"];';
    if(!testBefore.includes(oldCharge)||!testBefore.includes(oldCeiling)||!testBefore.includes(ratio)||!fixtureBefore.includes('"nativeRetention": {')||(hostBefore.match(/store::ToValue/g)??[]).length!==2||(hostBefore.match(/store::FromValue/g)??[]).length!==2)throw Error("exact backing/trait authority guard");
    const testAfter=testBefore.replace(oldCharge,"let charged=limits.max_allocation_bytes-control.allocation_remaining_bytes();").replace(oldCeiling,"let limits=SqliteDatabaseLimits{max_allocation_bytes:ceiling,..limits};").replace(ratio,ratio+'assert_eq!(ratio["ledger"],"native-backing-allocation");');
    const fixtureAfter=fixtureBefore.replace('"nativeRetention": {','"nativeRetention": {\n    "ledger": "native-backing-allocation",');
    const hostAfter=hostBefore.replaceAll("store::ToValue","semio_framework_value::ToValue").replaceAll("store::FromValue","semio_framework_value::FromValue");
    const pairs=[{path:testPath,before:testBefore,after:testAfter},{path:fixturePath,before:fixtureBefore,after:fixtureAfter},{path:hostPath,before:hostBefore,after:hostAfter}];
    writeFileSync(join(import.meta.dir,"native-backing-ledger-and-host-value-authority-pairs.json"),JSON.stringify({pairs},null,2)+"\n");
    for(const pair of pairs){if(readFileSync(join(repo,pair.path),"utf8")!==pair.before)throw Error("concurrent authority guard changed "+pair.path);}
    for(const pair of pairs)writeFileSync(join(repo,pair.path),pair.after);
    console.log("[DEBUG] Collection native-backing observation and actual Host value authority mounted paths=3");
}else throw Error("stage, mount, spelling or ledger-authority required");
