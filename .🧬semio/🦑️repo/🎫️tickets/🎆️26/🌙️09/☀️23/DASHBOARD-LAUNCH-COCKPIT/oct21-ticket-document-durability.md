# Ticket Document Durability

Actual native ticket probe 49840 reached all seven mounted scope laws: six passed and the lifecycle law failed at interactions.last().unwrap() after save/read. The canonical Rust encoder omitted interactions even though the decoder and service produced them. Go already persisted interactions. Both codecs and aggregate serializers also omitted declared parent and agent state. This is durable-state loss, not a fixture assertion defect.

Schema and the existing language-neutral codec document vectors were changed first. DURABLE-TICKET-STATE includes parent, ticket plan, two ordered interactions, file identities, two agent records, a fully populated nested plan, minimal nested step and an explicit empty plan object, with HTML-sensitive and Unicode text. The existing TypeScript/Ajv oracle now projects parent and agent state in schema order. The existing Rust rooted scope module adds an eighth law comparing actual codec output and model serialization with serde_json and replaying idempotence. The existing Go scope module adds a seventh law comparing semantic state and exact encoding/json.MarshalIndent bytes. These are runtime obligations, not static acceptance.

Rust now renders ordered arrays of object members and preserves interactions, parent and agents. Go now preserves parent and agents through the existing ordered member writer. Both aggregate serialization tags include durable parent and agent fields while rooted date, slug and filesystem paths stay derived. No external runtime dependency, alias, migration or source copy was added.

Actual repaired native and Go laws remain unrun. The paired integration dependency resolvers are independent, owned by Controls; Root changes no manifests or lockfiles. A new native ticket probe must capture the repaired sources. The four Go owning gates must include the added seventh ticket law, making fifteen new required Go bodies overall.

Public workspace node_modules Ajv2020 DATA validation accepted all eight codec documents against TicketDocument and the durable vector against the aggregate Ticket schema. Bare import and CommonJS-as-ESM attempts failed in the tool kernel; the standard createRequire route over the explicit public workspace module succeeded. This is schema DATA validation, not the Bun adapter or native/Go product body.

Independent Low accepted the durable writer/serializer/vector source slice. Aggregate schema was aligned with the same declared parent/agent ownership. The exact Go plan now explicitly retains the new seventh ticket law and all previous fourteen required Go bodies (fifteen total).

Public TypeScript parser accepted the updated independent adapter with zero syntax diagnostics. All thirteen curated codec/plan/report/schema paths have maximum absolute UTF-16 length146, within256. Static data is retained under generated; no compiler/body pass is implied.

A bounded static read of the actual canonical Go overlay owner confirms tests are virtual compiler inputs under the original module, and the owned command retains moduleRoot as its cwd. The new ../../tests fixture route resolves physically to the existing durable codec corpus from that cwd. No fixture is copied into ticket artifacts or a temporary mirrored source tree. This read is not actual Go execution.
