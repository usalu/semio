"""📦️ Stages a single pack wire for the PDF aggregate writer and retained receiver."""

from pathlib import Path
import hashlib
import json
import sys

TICKET=Path(__file__).resolve().parent
OUT=TICKET/"🗑️generated"/"vt-archive"
MUTATIONS=Path("🚪️io/💾️binary/🧬️mutations")
CODE=MUTATIONS/"🦀️.rs"
PROTOCOL=MUTATIONS/"📡️.protocol.semio"
TEST=Path("🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs")
FRAME="//#region 🧱️Framing\nconst MAX_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;\n\nfn malformed(detail: impl Into<String>) -> protocol::ProtocolError {\n    protocol::ProtocolError::Malformed { what: \"PDF mutation aggregate\", offset: 0, detail: detail.into() }\n}\n\nimpl OpBinary for PdfMutation {\n    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {\n        let bytes = store::pack_rt::encode_pack_value(&self.to_value());\n        if bytes.len() > MAX_PAYLOAD_BYTES {\n            return Err(protocol::ProtocolError::LimitExceeded(\"PDF mutation payload\"));\n        }\n        Ok(bytes)\n    }\n\n    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {\n        if bytes.len() > MAX_PAYLOAD_BYTES {\n            return Err(protocol::ProtocolError::LimitExceeded(\"PDF mutation payload\"));\n        }\n        let value = store::pack_rt::decode_pack_value(bytes).map_err(|error| malformed(error.to_string()))?;\n        Self::from_value(store::pack_rt::renormalize_whole_number_floats(value)).map_err(|error| malformed(error.to_string()))\n    }\n}\n//#endregion 🧱️Framing"


def candidates(base):
    code, protocol, test=((base/path).read_text(encoding="utf8") for path in (CODE,PROTOCOL,TEST))
    start=code.index("//#region 🧱️Framing")
    end=code.index("//#endregion 🧱️Framing")+len("//#endregion 🧱️Framing")
    new_code=(code[:start]+FRAME+code[end:]).replace("use protocol::OpBinary;","use protocol::OpBinary;\nuse semio_framework_value::{FromValue, ToValue};",1)
    new_code=new_code.replace("Generic binary framing and direct-owner registry for the visible PDF mutation aggregate.", "Pack binary framing for the PDF mutation aggregate and tags for its direct leaf codecs.",1)
    marker="# Real op frame:"
    assert protocol.count(marker)==1
    new_protocol=protocol.replace(marker,"# The aggregate PdfMutation OpBinary uses the Semio Pack wireValue document.\n# This framing describes only direct leaf codecs. Leaf op frame:",1)
    marker="        assert_eq!(bytes[0], store::pack_rt::OP_BINARY_FORMAT);"
    assert test.count(marker)==1
    new_test=test.replace(marker,'        assert!(bytes.starts_with(b"\\x89SPK\\r\\n\\x1a\\n"));',1)
    return {path:(before.encode("utf8"),after.encode("utf8")) for path,before,after in (
        (CODE,code,new_code),(PROTOCOL,protocol,new_protocol),(TEST,test,new_test)
    )}


def stage(base,action):
    files=candidates(base)
    receipt={}
    for index,(relative,(before,after)) in enumerate(files.items()):
        path=OUT/f"{index}{relative.suffix}"
        receipt[relative.as_posix()]={"overlay":path.name,"before":hashlib.sha256(before).hexdigest(),"after":hashlib.sha256(after).hexdigest()}
        if action=="--prepare":
            OUT.mkdir(parents=True,exist_ok=True)
            path.write_bytes(after)
        else:
            assert path.read_bytes()==after and (base/relative).read_bytes()==before
            assert len(str((base/relative).resolve()).encode("utf-16-le"))//2<=256
    witness=OUT/"receipt.json"
    if action=="--prepare":
        witness.write_text(json.dumps(receipt,ensure_ascii=False)+"\n",encoding="utf8")
    else:
        assert json.loads(witness.read_bytes())==receipt
        if action=="--apply-reviewed":
            for relative,(before,after) in files.items():
                assert (base/relative).read_bytes()==before
                (base/relative).write_bytes(after)
    print(f"[DEBUG] {action}: three PDF aggregate wire source paths")


if __name__=="__main__":
    assert sys.argv[2] in ("--prepare","--verify","--apply-reviewed")
    stage(Path(sys.argv[1]),sys.argv[2])
