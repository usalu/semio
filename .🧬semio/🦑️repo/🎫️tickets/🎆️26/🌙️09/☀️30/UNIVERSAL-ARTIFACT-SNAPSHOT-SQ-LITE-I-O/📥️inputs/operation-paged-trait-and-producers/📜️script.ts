const ticket=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O";
const input=`${ticket}/📥️inputs/operation-paged-trait-and-producers`;
const replication="🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs";
const plugin="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs";
const store="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs";
const pairs:{path:string,before:string,after:string}[]=[];
function replace(s:string,a:string,b:string,count=1){if(s.split(a).length!==count+1)throw Error(`exact bounded source span count ${count}`);return s.split(a).join(b);}
const before=await Bun.file(replication).text();
const signatures=`    /// ✍️ Appends the original operation directly into the caller's admitted backing and retains its accepted prefix on refusal.
    fn encode_op_into(&self,options:&crate::codec::PackEncodeOptions,output:&mut dyn operation_bytes::OperationByteOutput,control:&mut crate::value::NativeEncodeControl<'_>)->Result<(),crate::ProtocolError>;
    /// 🫳️ Reads the exact borrowed source under caller decode and canonical encode policies without a contiguous payload adapter.
    fn decode_op_span(source:crate::codec::ByteSpan<'_>,options:&crate::codec::PackDecodeOptions,canonical_options:&crate::codec::PackEncodeOptions,decoding:&mut crate::value::NativeDecodeControl<'_>,encoding:&mut crate::value::NativeEncodeControl<'_>)->Result<Self,crate::ProtocolError>;
`;
const after=replace(before,"    fn decode_op(bytes: &[u8]) -> Result<Self, crate::ProtocolError>;\n}","    fn decode_op(bytes: &[u8]) -> Result<Self, crate::ProtocolError>;\n"+signatures+"}");
pairs.push({path:replication,before,after});
const pluginBefore=await Bun.file(plugin).text();
const old=`            fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
                ::dsl::variants_binary::decode_op(bytes)
            }
`;
const methods=`            fn encode_op_into(&self,options:&::protocol::codec::PackEncodeOptions,output:&mut dyn ::protocol::operation_bytes::OperationByteOutput,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<(),::protocol::ProtocolError>{
                ::dsl::variants_binary::encode_op_into(self,options,output,control)
            }
            fn decode_op_span(source:::protocol::codec::ByteSpan<'_>,options:&::protocol::codec::PackDecodeOptions,canonical_options:&::protocol::codec::PackEncodeOptions,decoding:&mut ::semio_framework_value::NativeDecodeControl<'_>,encoding:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<Self,::protocol::ProtocolError>{
                ::dsl::variants_binary::decode_op_span(source,options,canonical_options,decoding,encoding)
            }
`.replace("source:::protocol","source: ::protocol");
pairs.push({path:plugin,before:pluginBefore,after:replace(pluginBefore,old,old+methods,4)});
const storeBefore=await Bun.file(store).text();
let storeAfter=storeBefore;
const storeMethods=`
    fn encode_op_into(&self,options:&crate::os_spr::codec::PackEncodeOptions,output:&mut dyn crate::os_spr::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),crate::os_spr::ProtocolError>{
        crate::os_dsl::variants_binary::encode_op_into(self,options,output,control)
    }
    fn decode_op_span(source:crate::os_spr::codec::ByteSpan<'_>,options:&crate::os_spr::codec::PackDecodeOptions,canonical_options:&crate::os_spr::codec::PackEncodeOptions,decoding:&mut semio_framework_value::NativeDecodeControl<'_>,encoding:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Self,crate::os_spr::ProtocolError>{
        crate::os_dsl::variants_binary::decode_op_span(source,options,canonical_options,decoding,encoding)
    }
`;
for(const name of ["OpsHeaderLine","CommandHeaderLine"]){
  const start=storeAfter.indexOf(`impl OpBinary for ${name} {`);
  const end=storeAfter.indexOf("\n}\n//#endregion",start);
  if(start<0||end<start)throw Error(`exact Store ${name} boundary`);
  storeAfter=storeAfter.slice(0,end)+storeMethods+storeAfter.slice(end);
}
pairs.push({path:store,before:storeBefore,after:storeAfter});
await Bun.write(`${input}/shared-source-trait-and-four-macros-held-pairs.json`,JSON.stringify({state:"HeldUntilEveryAuthoredImplementationAndPublicationCallerCloses",pairs,exclusions:["No production writes","No default Vec adapter","No compiler or runtime credit","Required handwritten and fixture codec implementations remain to be authored","ChildEmit and Store publication are not yet adopted"]},null,2)+"\n");
console.log("[DEBUG] "+JSON.stringify({heldPaths:pairs.length,sharedMacroCases:4,storeHeaderCases:2,productionMutations:0}));
