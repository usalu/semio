import json, re, sys
V = "️"
F = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🚶️hydration-step/🦀️.rs"
def child(e, name, text):
    return {"dir": e + V + name, "doc": "//! %s%s %s See [`super`]." % (e, V, text)}
g = {
 1: child("📦", "pack-scan-and-fold", "Pack scan and fold phases: the pack scan, decode, begin, fold, initialization catalog and genesis binding of the hydration step."),
 2: child("✉", "edit-decoding", "Edit decoding phases: the envelope admission, cursor building and the forward, inverse and metadata decoding of the hydration step."),
 3: child("💧", "change-hydration", "Change hydration phases: the changes, checkpoints, alternatives and pins hydration of the hydration step."),
 4: child("🔁", "replay-and-targets", "Replay and target phases: the replay preparation and cursor, inverse and message adoption, replay settling and the target phases of the hydration step."),
 5: child("🏁", "retirement-and-finish", "Retirement and finish phases: the history and pack retirement, the envelope close and the finish of the hydration step."),
}
rows = [
 (29,"ScanPack",1,"🔍","Scans the retained pack for the next record."),(46,"DecodePack",1,"🧬","Decodes the scanned pack record."),(56,"Begin",1,"🚦","Begins the hydration."),(79,"Fold",1,"🪄","Folds the decoded records into the history."),
 (106,"AdmitInitializationCatalog",1,"📇","Admits the initialization catalog."),(116,"BindGenesis",1,"🌱","Binds the genesis snapshot."),
 (155,"AdmitEnvelope",2,"🛂","Admits one edit envelope."),(165,"BuildAppliedCursor",2,"🧭","Builds the applied or redo cursor."),(180,"BeginEdit",2,"✏","Begins one edit."),(220,"DecodeForward",2,"➡","Decodes the forward or inverse mutation."),
 (242,"DecodeMetadata",2,"🏷","Decodes the edit metadata."),(272,"FinishEdit",2,"✔","Finishes one edit."),
 (294,"HydrateChanges",3,"🔄","Hydrates the change records."),(310,"HydrateCheckpoints",3,"📍","Hydrates the checkpoints."),(336,"HydrateAlternatives",3,"🔀","Hydrates the alternatives."),(352,"HydratePins",3,"📌","Hydrates the pins."),
 (387,"PrepareReplay",4,"🎞","Prepares the replay."),(408,"ReplayCursor",4,"⏯","Advances the replay cursor."),(427,"AdoptInverse",4,"↩","Adopts the replayed inverse."),(439,"AdoptMessages",4,"💬","Adopts the replayed messages."),
 (465,"SettleReplay",4,"⚖","Settles the replay."),(480,"BeginTargets",4,"🎯","Begins the target phases."),(509,"DecodeTarget",4,"🎙","Decodes one target."),(529,"TargetInputs",4,"📥","Collects the target inputs."),(566,"SeedApplied",4,"🌾","Seeds the applied or redo target."),
 (590,"RetireHistory",5,"🧹","Retires the history runtime."),(603,"RetirePack",5,"🗑","Retires the pack."),(612,"CloseEnvelopeRuntime",5,"🔒","Closes the envelope runtime."),(625,"CloseEnvelopeOwners",5,"🔐","Closes the envelope owners."),(642,"Finish",5,"🏆","Finishes the hydration."),
]
def snake(s): return re.sub(r"(?<!^)([A-Z])", lambda m: "_" + m.group(1), s).lower()
used=set(); arms=[]
for line,name,gr,em,text in rows:
    assert em not in used, em; used.add(em)
    arms.append({"line": line, "name": "step_" + snake(name), "doc": "%s %s" % (em, text), "child": g[gr], "allow_returns": True, "binds": {}})
hdr = "impl<P, M> RetainedPersistedDocumentHydration<P, M>\nwhere\n    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,\n    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,"
spec = {"file": F, "function": "step", "match_line": 28, "types": {"grant": "RetainedCloneGrant"}, "ret": "PersistedDocumentHydrationStep<P, M>",
  "method": {"self_param": "&mut self", "impl_header": hdr, "call": "{name}({args})", "vis": "pub(super)"},
  "prelude_if": {"regex": r"to_uri\(|parse_uri", "line": "use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};"},
  "arms": arms}
json.dump(spec, open(sys.argv[1], "w", encoding="utf8"), ensure_ascii=False, indent=1)
