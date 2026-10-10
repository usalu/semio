# Actual Editable SVG End-User Ingress Frontier

Read-only original source audit on 2026-10-09. No new import command or runtime acceptance is claimed.

The actual Draw editor registers encoded PNG import with its dedicated file-open effect, retained image admission, semantic ImportImageAsset/CreateLayer mutations and selection effect. The UI lane confirms that this route intentionally covers PNG MIME input only. Its catalogue action is internal and the dedicated end-user control requests `image/png,.png`.

Editable SVG already has an actual registered `SvgIntoDraw` deserializer and bilingual-independent document fixtures. It preserves hierarchy, paths, linear gradients, transforms and supported styling. However `drawing_io()` advertises no import formats and no dedicated editable SVG file-open action appears in the actual editor catalogue. The existence of the deserializer alone does not establish end-user ingress.

The existing `SvgImportJob::new` synchronously parses the entire XML document, scans the graph and clones gradient definitions. Its work budget counts visited nodes while geometry conversion and style construction can process large node bodies. `cancel` and failure synchronously clear pending nodes, groups, gradients and the partial document. The async deserializer loops `step(64)` without an await or host scheduling boundary. These are actual unsupported cooperative scheduling/physical custody gates, rather than evidence that SVG editing can safely be exposed by adding an action label.

The correct implementation must retain the genuine file payload and parsed graph behind the existing original-source issuer, fund lexical/attribute/path parsing and hierarchy construction from current grants, keep every cancelled or rejected prefix until funded retirement, and publish editable layers through semantic mutations with inverse diffs. A dedicated bilingual file-open control must use that actual job. Merely advertising an import format, synchronously calling the old deserializer, clearing its graph on cancellation, or replacing the document through a history reset would not satisfy the intended editing journey.

Source owners inspected: Draw `editor/🦀️.rs`, `editor/commands/import-image/🦀️.rs`, `io/import/deserializers/artifacts/svg/1.1/any/🦀️.rs`, its `document/🦀️.rs` and native/portable fixtures. This audit ran no current SVG native or browser import journey. Editable SVG ingress remains an explicit product gap in this ticket.
