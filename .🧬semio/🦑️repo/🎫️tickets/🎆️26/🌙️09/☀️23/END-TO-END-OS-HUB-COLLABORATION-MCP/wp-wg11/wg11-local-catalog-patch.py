#!/usr/bin/env python3
"""🗂️ WG11 session 15 — the wgpu shell serves the local document catalog (SH2's route B, its native half) instead of refusing it.

Measured (live tree 2026-09-29 19:3x): `handle_replay_shell_command` answered every `os.local-catalog.*` with the replay
refusal `local-catalog-unavailable` — so on the native winit shell a Home persist/bind/import commit never kept anything,
although that shell has a data folder and the same kernel folder backbone React's worker persists through. Two defects in the
shared schema surfaced on the way: ShellHost's catalog archive wrote `admittedAtMs` as an f64 (`encodePackValue` of a JS
number) where the schema (`LocalDocument.admitted_at_ms: u64`) is an exact `uint` — Rust's `FromValue` refuses that float and
React's decoder refused Rust's `uint`, so neither shell could read the other's catalog; and an id `..` / `.` resolved the default
target to `<dataDir>/os/local-documents/..` — the catalog facet's own folder.

Set:
  1. `🏛️ShellHost/🗂️local-catalog/🔣️.json` (NEW) — the schema-first lane vocabulary both shells read: commands, bounds, en + de
     notices + fault-code prefix, shared vectors (19 admissions, 2 catalog archives; archive bytes from the TS encoders).
  2. `🏛️ShellHost/🗂️local-catalog/🟦️.ts` — reads 1 (API unchanged); archive writes `admittedAtMs` as `packUInt`, the decoder
     reads a `uint` carrier or an envelope's JSON number; ids `.` / `..` refused.
  3. wgpu shell — `🗂️LocalCatalog` region: the vocabulary, `local_catalog_admission_v1` (ShellHost's twin), the catalog archive
     codec, and on native the lane: facet opened at boot from `<S_DATA_DIR>/os` (in memory without a data folder), admit =
     write into the document's folder lane + read back identically + record `admitLocalDocument` (a session that is gone before
     the write cancels it), retire = record `retireLocalDocument`, the landing app handed every kept document its instance was not
     handed yet (`applyLocalCatalogDocument`); one job at a time on the I/O lane with the vocabulary's deadlines; every outcome a
     localized notice under its code. The browser shell (no data folder) answers `no-data-folder` / `document-unknown`. Replay
     reason `local-catalog-unavailable` removed (no route raises it any more) from the enum and `📣️replay-refusal/🔣️.json`.
  4. Laws: Rust (hub-projection-workspace) — vocabulary; admission vectors; archive vectors both ways; native end to end on a real
     temp data folder (bare shell refusal, keeping/kept, bytes on disk, a fresh shell lists it, retire in German, unknown retire,
     incomplete request, cancelled admission writes nothing); the unserved-replay law without the old reason. TS (engine-contract)
     — the same vocabulary, admission and archive vectors against ShellHost's lane.

Crates: semio-framework-os-renderer-wgpu (native + wasm32-unknown-unknown). TS: os (ShellHost local-catalog, engine-contract).
Order: after SH2's P2 set if both ride one round (dry-run clean on live and on live + P2's payload; no shared hunk).
Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/local-catalog/` and applies (every
anchor asserted exactly once; refuses when the new file exists); `--revert` restores the backups and removes the new file;
`--base <dir>` dry-runs against another tree root (e.g. a scratch mirror with P2 applied).
"""

import difflib
import json
import shutil
import sys
from pathlib import Path

LIVE = Path("/Users/ueli/Documents/semio")
ROOT = Path(sys.argv[sys.argv.index("--base") + 1]) if "--base" in sys.argv else LIVE
PAYLOAD = Path("/Users/ueli/Documents", "semio/.tmp-ticket/wp-wg11/local-catalog")
BACKUP = LIVE / ".🧬semio/🌐hub/s14-wg11-backup/local-catalog"
ENGINE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
VOCABULARY = f"{ENGINE}/🧱️elements/🏛️ShellHost/🗂️local-catalog/🔣️.json"
LANE_TS = f"{ENGINE}/🧱️elements/🏛️ShellHost/🗂️local-catalog/🟦️.ts"
REFUSAL_JSON = f"{ENGINE}/🧱️elements/🏛️ShellHost/📣️replay-refusal/🔣️.json"
SHELL = f"{ENGINE}/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
SHELL_LAWS = f"{ENGINE}/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs"
CONTRACT_TS = f"{ENGINE}/🧪️tests/🔬️engine-contract/🟦️.ts"

REGION = (PAYLOAD / "rust-region.rs").read_text(encoding="utf-8")
LAWS = (PAYLOAD / "rust-laws.rs").read_text(encoding="utf-8")
TS_LAW = (PAYLOAD / "ts-law.ts").read_text(encoding="utf-8")

SHELL_EDITS = [
    (
        "use semio_framework_os_config::opening_config::{\n",
        "use semio_framework_os_config::mutations::{LocalCatalog, LocalDocument, LocalDocumentStorage};\nuse semio_framework_os_config::opening_config::{\n",
    ),
    ("    UnroutedCommand,\n    LocalCatalogUnavailable,\n}\n", "    UnroutedCommand,\n}\n"),
    (
        """    pub(crate) const ALL: [Self; 9] =
        [Self::SignInRequired, Self::SpaceRequired, Self::SpaceIndexRequired, Self::InvalidRequest, Self::RouterNotReady, Self::OpenRejected, Self::ViewOnlyAccess, Self::UnroutedCommand, Self::LocalCatalogUnavailable];
""",
        """    pub(crate) const ALL: [Self; 8] = [Self::SignInRequired, Self::SpaceRequired, Self::SpaceIndexRequired, Self::InvalidRequest, Self::RouterNotReady, Self::OpenRejected, Self::ViewOnlyAccess, Self::UnroutedCommand];
""",
    ),
    ("    Err(invalid)\n}\n//#endregion 📣️ReplayRefusal\n", "    Err(invalid)\n}\n//#endregion 📣️ReplayRefusal\n" + REGION),
    (
        """    pub identity_env: Option<IdentityEnv>,
    /// 🎭️ Per-process session id""",
        """    pub identity_env: Option<IdentityEnv>,
    /// 🗂️ The host-owned local document catalog lane (`os.local-catalog.*`, route B) — [`LocalCatalogLaneV1`].
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) local_catalog: LocalCatalogLaneV1,
    /// 🎭️ Per-process session id""",
    ),
    (
        """            identity_env: None,
            shell_session_id: mint_shell_session_id(),
""",
        """            identity_env: None,
            #[cfg(not(target_arch = "wasm32"))]
            local_catalog: LocalCatalogLaneV1::default(),
            shell_session_id: mint_shell_session_id(),
""",
    ),
    (
        """        self.sync_session_chrome();
        #[cfg(not(target_arch = "wasm32"))]
        self.bootstrap_identity();
        #[cfg(target_arch = "wasm32")]
        {
            self.identity_env = resolve_identity_env();
        }
""",
        """        self.sync_session_chrome();
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.bootstrap_identity();
            self.open_local_catalog();
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.identity_env = resolve_identity_env();
        }
""",
    ),
    (
        """        changed |= self.pump_hub_artifact_creation().await;
        changed |= self.pump_hub_check_in().await;
""",
        """        changed |= self.pump_hub_artifact_creation().await;
        changed |= self.pump_local_catalog().await;
        changed |= self.pump_hub_check_in().await;
""",
    ),
    (
        """    /// relay or the creation door; an `os.*` command this shell cannot serve is refused out loud ([`Self::refuse_replay`]) —
    /// the local studio catalog (`os.local-catalog.*`) until route B's native half lands, any unknown one as unrouted.
""",
        """    /// relay, the creation door or the local studio catalog's lane ([`Self::replay_local_catalog`]); an `os.*` command this shell
    /// cannot serve is refused out loud as unrouted ([`Self::refuse_replay`]).
""",
    ),
    (
        """            } else if action_id.starts_with("os.local-catalog.") {
                Some(ReplayRefusalReasonV1::LocalCatalogUnavailable)
""",
        """            } else if action_id.starts_with("os.local-catalog.") {
                self.replay_local_catalog(action_id, args_json.as_ref())
""",
    ),
]

LAW_EDITS = [
    (
        """/// 📣️ A guest replay this shell cannot serve is refused OUT LOUD — the vocabulary's warning notice in the shell's tongue with
/// its fault code — never dropped: the local studio catalog (route B has no native half yet), an unknown `os.*` command or
/// directory verb, and a space artifact creation nobody signed in for, or from no mounted Space index.
""",
        """/// 📣️ A guest replay this shell cannot serve is refused OUT LOUD — the vocabulary's warning notice in the shell's tongue with
/// its fault code — never dropped: a local-catalog verb its lane does not serve, an unknown `os.*` command or directory verb,
/// and a space artifact creation nobody signed in for, or from no mounted Space index.
""",
    ),
    (
        """    use ReplayRefusalReasonV1::{InvalidRequest, LocalCatalogUnavailable, SignInRequired, SpaceIndexRequired, UnroutedCommand};
""",
        """    use ReplayRefusalReasonV1::{InvalidRequest, SignInRequired, SpaceIndexRequired, UnroutedCommand};
""",
    ),
    (
        """    let cases: [(&str, &str, bool, &[(&str, &str)], ReplayRefusalReasonV1); 7] = [
        ("en", "os.local-catalog.admit", false, &[("documentId", "studio-1")], LocalCatalogUnavailable),
        ("de", "os.local-catalog.open", true, &[("documentId", "studio-1")], LocalCatalogUnavailable),
        ("de", "os.local-catalog.retire", true, &[("documentId", "studio-1")], LocalCatalogUnavailable),
""",
        """    let cases: [(&str, &str, bool, &[(&str, &str)], ReplayRefusalReasonV1); 5] = [
        ("de", "os.local-catalog.open", true, &[("documentId", "studio-1")], UnroutedCommand),
""",
    ),
    (
        """/// 🌱️ Only a canonical choice of one kind the space's ready catalog offers becomes a creation""",
        LAWS + """/// 🌱️ Only a canonical choice of one kind the space's ready catalog offers becomes a creation""",
    ),
]

CONTRACT_EDITS = [
    (
        """import { REPLAY_REFUSAL_LABELS_V1, replayRefusalCodeV1, replayRefusalNoticeTextV1, type ReplayRefusalReasonV1 } from "../../🧱️elements/🏛️ShellHost/📣️replay-refusal/🟦️.ts";
""",
        """import { REPLAY_REFUSAL_LABELS_V1, replayRefusalCodeV1, replayRefusalNoticeTextV1, type ReplayRefusalReasonV1 } from "../../🧱️elements/🏛️ShellHost/📣️replay-refusal/🟦️.ts";
import localCatalogVocabulary from "../../🧱️elements/🏛️ShellHost/🗂️local-catalog/🔣️.json";
import { decodeLocalCatalogArchiveV1, localCatalogAdmissionV1, localCatalogArchiveV1, localCatalogNoticeCodeV1, localCatalogNoticeTextV1, type LocalCatalogNoticeV1 } from "../../🧱️elements/🏛️ShellHost/🗂️local-catalog/🟦️.ts";
""",
    ),
    (
        """      expect([replayRefusalNoticeTextV1(reason, vector.locale), replayRefusalCodeV1(reason)]).toEqual([vector.text, vector.code]);
    }
  });
});
""",
        """      expect([replayRefusalNoticeTextV1(reason, vector.locale), replayRefusalCodeV1(reason)]).toEqual([vector.text, vector.code]);
    }
  });
});
""" + TS_LAW,
    ),
]


def replaced(name: str, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def refusal_vocabulary(source: str) -> str:
    vocabulary = json.loads(source)
    if "local-catalog-unavailable" not in vocabulary["reasons"]:
        sys.exit("replay-refusal vocabulary: local-catalog-unavailable already gone")
    del vocabulary["reasons"]["local-catalog-unavailable"]
    reasons = vocabulary["reasons"]
    vocabulary["vectors"] = [
        {"reason": "view-only-access", "locale": "de", "text": reasons["view-only-access"]["de"], "code": "shell.replayShellCommand.view-only-access"},
        {"reason": "router-not-ready", "locale": "en", "text": reasons["router-not-ready"]["en"], "code": "shell.replayShellCommand.router-not-ready"},
    ] + [vector for vector in vocabulary["vectors"] if vector["reason"] != "local-catalog-unavailable"]
    return json.dumps(vocabulary, indent=2, ensure_ascii=False) + "\n"


def lane_module(source: str) -> str:
    base = (PAYLOAD / "base.ts").read_text(encoding="utf-8")
    if source != base:
        sys.exit(f"{LANE_TS}: the live module is not the set's base (a peer changed it) — rebase `local-catalog/new.ts` on it")
    return (PAYLOAD / "new.ts").read_text(encoding="utf-8")


PLAN = [
    (SHELL, lambda source: replaced(SHELL, source, SHELL_EDITS)),
    (SHELL_LAWS, lambda source: replaced(SHELL_LAWS, source, LAW_EDITS)),
    (CONTRACT_TS, lambda source: replaced(CONTRACT_TS, source, CONTRACT_EDITS)),
    (REFUSAL_JSON, refusal_vocabulary),
    (LANE_TS, lane_module),
]


def main():
    if "--revert" in sys.argv:
        for relative, _ in PLAN:
            backup = BACKUP / relative
            if not backup.exists():
                sys.exit(f"no backup for {relative}")
            shutil.copyfile(backup, LIVE / relative)
        (LIVE / VOCABULARY).unlink(missing_ok=True)
        print("REVERTED: every file restored from its backup, the new vocabulary removed")
        return
    write = "--write" in sys.argv
    if (ROOT / VOCABULARY).exists():
        sys.exit(f"{VOCABULARY} exists already")
    planned = []
    for relative, transform in PLAN:
        before = (ROOT / relative).read_text(encoding="utf-8")
        planned.append((relative, before, transform(before)))
    vocabulary = (PAYLOAD / "vocabulary.json").read_text(encoding="utf-8")
    for relative, before, after in planned:
        name = "/".join(Path(relative).parts[-3:])
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), name, f"{name} (patched)", n=1))
    print(f"+++ NEW {VOCABULARY} ({len(vocabulary)} bytes)")
    if write:
        if ROOT != LIVE:
            sys.exit("--write only applies to the live tree")
        for relative, before, _ in planned:
            backup = BACKUP / relative
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for relative, _, after in planned:
            (LIVE / relative).write_text(after, encoding="utf-8")
        (LIVE / VOCABULARY).write_text(vocabulary, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} edited + 1 new file (crate: semio-framework-os-renderer-wgpu; TS: os ShellHost local-catalog + engine-contract law)")


if __name__ == "__main__":
    main()
