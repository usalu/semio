/** 🎟️ Composes the direct mounted picker hooks with paid adoption and cancellation-first media turns. */
import { readFileSync, renameSync, writeFileSync } from "node:fs";
const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin" + "/";
const generated = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR" + "/🗑️generated/";
const held = JSON.parse(readFileSync(generated + "raw-picker-mounted-overlay.json", "utf8")).candidate;
const media = root + "⚛️reactor/🏃️runtime/🧰️ops/🎞️raw-media/🦀️.rs";
const outbox = root + "⚛️reactor/🏃️runtime/🧰️ops/🎞️raw-media/📤️picker.rs";
const owner = root + "🧵️instance/🔌️plugin-app-impl/🎟️picker-publication/🦀️.rs";
const turn = root + "⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs";
function one(text: string, before: string, next: string) {
    if (text.split(before).length !== 2) throw Error("Shared picker anchor changed: " + before);
    return text.replace(before, () => next);
}
const mediaLive = readFileSync(media, "utf8");
let mediaNext = mediaLive;
if (mediaLive.includes("app.original_picker()")) {
    mediaNext = one(mediaLive, "app.original_picker().and_then(|owner| owner.status().document)", "app.original_media_live_document()");
    mediaNext = one(mediaNext, `        let status = app.original_picker().map(|owner| owner.status());
            let live = status.as_ref().and_then(|status| status.document.as_ref());
            let format_closed = status.as_ref().is_none_or(|status| status.format_closed);
            if let Some(ack) = slot.raw_media.poll(live, format_closed, cx)? {`,
        `        let live = app.original_media_live_document();
            if let Some(ack) = slot.raw_media.poll(live.as_ref(), app.original_media_format_closed(), cx)? {`);
    mediaNext = one(mediaNext, "priority: Option<u32>, _identity:", "priority: Option<u32>, identity:");
    mediaNext = one(mediaNext, "active.app.advance_original_picker(&mut slot.raw_media.picker, cx).await?;",
        "active.app.advance_original_media_format(&mut slot.raw_media.picker, identity, cx).await?;");
    mediaNext = one(mediaNext, `        let live = if let Some(owner) = app.original_picker() {
                owner.observe_publication(&mut slot.raw_media.picker, cx)?;
                owner.status().document
            } else { None };`,
        `        app.observe_original_media_commit(&mut slot.raw_media.picker, cx)?;
            let live = app.original_media_live_document();`);
} else if (!mediaLive.includes("active.app.advance_original_media_format") || !mediaLive.includes("app.observe_original_media_commit")) {
    throw Error("Shared media hooks changed");
}

const outboxLive = readFileSync(outbox, "utf8");
if (!outboxLive.includes("owner.pending_effect()") || !held[outbox].includes("app.original_picker_effect()")) throw Error("Picker outbox ownership changed");
const ownerLive = readFileSync(owner, "utf8");
const ownerTrait = ownerLive.indexOf("use semio_plugin_reactor_app::{OriginalPickerPublication, OriginalPickerStatus};");
const observer = ownerLive.indexOf("    fn observe_publication(", ownerTrait);
const observerEnd = ownerLive.indexOf("\n}\n\nimpl<A:", observer);
const inherent = ownerLive.indexOf(" {\n", observerEnd + 4) + 3;
if (ownerTrait < 0 || observer < 0 || observerEnd < 0 || inherent <= observerEnd) throw Error("The picker owner changed");
const ownerNext = (ownerLive.slice(0, ownerTrait).trimEnd() + "\n\n" +
    ownerLive.slice(observerEnd + 4, inherent) +
    ownerLive.slice(observer, observerEnd).replace("fn observe_publication(", "pub(super) fn observe_picker_commit(") +
    "\n\n" + ownerLive.slice(inherent)).replace("self.observe_publication(picker, cx)?;", "self.observe_picker_commit(picker, cx)?;");
if (!ownerNext.includes("pub(super) fn close_picker(")) throw Error("Missing paid picker close");

const turnLive = readFileSync(turn, "utf8");
const priority = "    let media_priority = ";
const receiving = "    advance_original_receiving(";
const begin = turnLive.indexOf(priority), end = turnLive.indexOf(receiving, begin);
if (begin < 0 || end < 0) throw Error("The media turn changed");
const turnNext = turnLive.slice(0, begin) + `    let media_ingress = semio_plugin_runtime_ops::ops_raw_media::drive_raw_media(runtime, events, command_page, &close_instances, identity, cx)?;
    let media_priority = match media_ingress.as_ref() {
        Some(semio_framework::kernel::CommandIngressStatus::OriginalMediaAck(ack))
            if ack.phase == semio_framework::kernel::OriginalMediaWirePhase::Cancel => Some(ack.loan.instance_id),
        _ => close_instances.first().copied(),
    };
    let media_format_runnable = command_page.is_none()
        && semio_plugin_runtime_ops::ops_raw_media::drive_raw_media_format(runtime, &close_instances, media_priority, identity, cx).await?;
    let command_ingress = match media_ingress {
        Some(status) => {
            drive_command_ingress(runtime, &mut None, close_instances, &mut effects, identity, cx).await?;
            status
        }
        None => drive_command_ingress(runtime, command_page, close_instances, &mut effects, identity, cx).await?,
    };
` + turnLive.slice(end);

for (const [index, [file, previous, updated]] of [[media, mediaLive, mediaNext], [outbox, outboxLive, held[outbox]],
    [owner, ownerLive, ownerNext], [turn, turnLive, turnNext]].entries()) {
    const temporary = generated + "picker-direct-app-" + index;
    writeFileSync(temporary, updated);
    if (readFileSync(file, "utf8") !== previous) throw Error("Shared picker source changed: " + file);
    renameSync(temporary, file);
    console.log("[DEBUG]", file, Buffer.byteLength(updated) - Buffer.byteLength(previous));
}
