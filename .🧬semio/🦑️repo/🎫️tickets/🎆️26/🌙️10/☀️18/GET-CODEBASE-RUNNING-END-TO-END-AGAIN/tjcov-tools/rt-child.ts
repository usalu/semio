import { instance } from "./rt-next.ts";
const cs = instance.lastIndexOf("async fn publish_mounted_typed_child_operation_unit(");
const ce = cs < 0 ? -1 : instance.indexOf("fn publish_mounted_typed_operation_unit(", cs);
console.log("child start", cs, "end", ce);
if (cs >= 0 && ce >= 0) {
  const child = instance.slice(cs, ce);
  for (const s of ["PendingChildGroupPublicationPhase::Ready", "captured_child_content_generation", "captured.identity_digest()", "try_claim_publication", "begin_dispatch", "&pending.child_emits", "dispatch_emit_group(", "TypedOperationResultLane::Chil"]) console.log(child.includes(s) ? "ok  " : "MISS", s);
}
