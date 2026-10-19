import { instance } from "./rt-next.ts";
const ps = instance.lastIndexOf("fn publish_mounted_typed_operation_unit");
const pe = instance.indexOf("pub async fn advance_typed_operation_publication_one", ps);
const pub = instance.slice(ps, pe);
console.log("publisher bytes", pub.length);
for (const n of ["pending_artifact_publication", "begin_apply_batch", "advance_apply_batch", "ArtifactStoreOneItemAdvance::Published", "pending_publication_outcome.pending()", ".await"]) console.log(n, pub.split(n).length - 1);
for (const m of instance.matchAll(/begin_apply_batch/g)) console.log("begin_apply_batch @", m.index, JSON.stringify(instance.slice(m.index! - 90, m.index! + 40)));
for (const n of ["admit_apply_batch", "spawn_task", "typed_effect_outbox.push", "typed_event_outbox.push", "typed_ui_outbox.push", "pending_publication_outcome", ".apply_one(", "dispatch_emit_group(", "async fn", "pending.begin_close()", "fn publish_mounted_typed_operation_unit"]) console.log(n, pub.split(n).length - 1);
for (const m of pub.matchAll(/pending_publication_outcome|spawn_task/g)) console.log(m[0], "@", m.index, JSON.stringify(pub.slice(m.index! - 80, m.index! + 90)));
