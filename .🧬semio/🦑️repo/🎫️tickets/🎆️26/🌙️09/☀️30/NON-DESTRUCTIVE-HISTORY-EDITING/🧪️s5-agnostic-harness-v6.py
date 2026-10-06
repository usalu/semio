#!/usr/bin/env python3
"""🧪️ S5-AGNOSTIC harness v6 (found by the flow + cad family, 18:46):
1. CONTROL FOR A FAILED RELOAD — cad `create-energy-model` (a parent-lane leaf whose inputs name an owned child) failed with "the
   archive load ends Fault … closure-rejected (Incomplete)" after the edit, and nothing told whether the EDIT or the SEED (the harness
   seeds through the store alone, which opens no member) broke the ownership closure. Now a failed reload of the edited document is
   followed by the same save → load of a freshly seeded, unedited instance: if that fails too the case is skipped naming the seed; if it
   succeeds the failure is the edit's, and the message carries the drafted change.
2. EXAMPLES THROUGH THE APP'S OWN ROUTE — flow ships command scripts (`🎮️.cmd.semio`) as examples; the reload law loaded every example
   body as document text ("expected Record, found Absent"). An example now reaches the instance through `setActiveExample{exampleId}`,
   the route the shell uses (the app's own action, else the framework's catalogue route), published to completion.
3. The pump's timeout says what is still pending (history work, typed operations) instead of only naming the session.
Anchored on the exact post-v5 text, count-asserted, one write; idempotent. Usage: [--apply] [--preview <file>]"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"

EDITS = [
    ("""    Err(format!("the session never settled within 60 s; it rests at {:?} (pending work: {})", app.time_travel.status().map(|status| status.stage), app.time_travel.has_pending_work()))""",
     """    Err(format!(
        "the instance never settled within 60 s; time travel rests at {:?} (pending history work: {}, pending typed operations: {})",
        app.time_travel.status().map(|status| status.stage),
        app.time_travel.has_pending_work(),
        app.has_pending_typed_operations()
    ))"""),
    ("""        (Err(reason), _, _, _) | (_, Err(reason), _, _) | (_, _, Err(reason), _) | (_, _, _, Err(reason)) => AcceptanceVerdict::Fail(reason),""",
     """        (Err(reason), _, _, _) | (_, Err(reason), _, _) | (_, _, Err(reason), _) => AcceptanceVerdict::Fail(reason),
        (_, _, _, Err(reason)) => match acceptance_seeded::<A, M>(manifest, &case.base, &seed, true).await {
            Err(fault) => AcceptanceVerdict::Fail(format!("the edited document does not survive save and load ({reason}) and its control does not seed: {}", fault.reason())),
            Ok(mut control) => {
                let unedited = acceptance_reloaded_head(&control, manifest).await;
                acceptance_close(&mut control).await;
                match unedited {
                    Err(seeded) => AcceptanceVerdict::SkipCase(format!("the seeded document does not survive save and load before any edit ({seeded}): a seed through the store alone does not complete what this operation owns")),
                    Ok(_) => AcceptanceVerdict::Fail(format!("after {} = {:?} the edited document does not survive save and load although the seeded one does: {reason}", change.0, change.1)),
                }
            }
        },"""),
    ("""    let mut documents = vec![("initial".to_string(), None)];
    for example in examples {
        let body = example.document();
        let text = if body.trim_start().starts_with('{') {
            semio_framework_pack_json::parse(&body, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|json| acceptance_dsl_of::<A>(semio_framework_pack_json::to_dsl_value(&json)))
        } else {
            Some(body)
        };
        documents.push((format!("example {}", example.id()), text));
    }
    let mut reloaded_documents = 0;
    for (name, text) in documents {""",
     """    let mut documents = vec![("initial".to_string(), None)];
    documents.extend(examples.iter().map(|example| (format!("example {}", example.id()), Some(example.id().to_string()))));
    let mut reloaded_documents = 0;
    for (name, example) in documents {"""),
    ("""            if let Some(text) = text {
                let mut files = saved.document_text().await.map_err(|fault| format!("the document does not print: {fault:?}"))?;
                files.dsl = text;
                artifact_app_laws::load_document_text(&mut saved, &files).await.map_err(|fault| format!("the document does not load: {fault:?}"))?;
            }""",
     """            if let Some(example) = example {
                saved.store.set_local_actor_id(Some(ACCEPTANCE_ACTOR.to_string())).map_err(|error| format!("the actor is refused: {error:?}"))?;
                acceptance_verb(&mut saved, CATALOGUE_EXAMPLE_ACTION_ID, vec![("exampleId", DslValue::String(example))]).await.map_err(|refusal| format!("the app's own example route does not load it: {refusal}"))?;
                acceptance_pump(&mut saved, |app| !app.has_pending_typed_operations()).await.map_err(|reason| format!("the example never finishes loading: {reason}"))?;
            }"""),
    ("""/// 🔂️ LAW (design §20.15): a document survives save → fresh load. Every document the app ships (its initial document and each of
/// `examples`) saved as its recursive archive""",
     """/// 🔂️ LAW (design §20.15): a document survives save → fresh load. Every document the app ships (its initial document and each of
/// `examples`, loaded through the app's own example route `setActiveExample{exampleId}`) saved as its recursive archive"""),
]


def main() -> None:
    with open(PATH, encoding="utf-8") as handle:
        before = handle.read()
    if "the seeded document does not survive save and load before any edit" in before:
        print("SKIP: already carries harness v6")
        return
    if "swept only on a shipped document" not in before:
        sys.exit("ORDER: apply harness v5 first")
    for index, (old, _) in enumerate(EDITS):
        if before.count(old) != 1:
            sys.exit(f"ANCHOR #{index}: {before.count(old)} matches: {old[:90]!r}")
    after = before
    for old, new in EDITS:
        after = after.replace(old, new)
    if "--preview" in sys.argv:
        open(sys.argv[sys.argv.index("--preview") + 1], "w", encoding="utf-8").write(after)
    if "--apply" not in sys.argv:
        print(f"WOULD apply harness v6 ({len(EDITS)} hunks, {after.count(chr(10)) - before.count(chr(10)):+d} lines)")
        return
    with open(PATH, encoding="utf-8") as handle:
        if handle.read() != before:
            sys.exit("RACE: the harness changed while staging")
    with open(PATH, "w", encoding="utf-8") as handle:
        handle.write(after)
    print("WROTE")


if __name__ == "__main__":
    main()
