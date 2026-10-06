"""🪙️ Outcome-code vocabulary wave (S5-PUZZLE, design §22.5 + §22.13): two codes join the closed framework vocabulary.

- `mutation.precondition-drifted` — Warning: the mutation applied, a precondition it was recorded under no longer holds.
- `mutation.inverse-refused` — Fatal: the mutation cannot be reversed on the state it replays onto.

One hunk per site that enumerates the vocabulary, schema and fixture first, then the Rust table and its TypeScript twin,
then the words (guest history body, React label keys and both locale bundles). All or nothing: every anchor must resolve
exactly once (or its replacement must already be present) before any file is written. `--check` writes nothing and prints
the pending hunks. Land under `landing` + `serve` while `🗑️generated/coord/foundation.status` is GREEN.

Run: `python3 <this file> [--check]`.
"""

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
FW = "🧰️framework/🔨️modules"
OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
MUTATION = f"{FW}/📡️replication/🎮️mutation/🦀️.rs"
SHELL = f"{OSM}/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
BUNDLES = f"{FW}/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts"

HUNKS = []


def hunk(path, old, new):
    HUNKS.append((path, old, new))


# 🧫️ The language-agnostic vocabulary and the two schemas that enumerate it.
hunk(
    f"{FW}/📡️replication/🎮️mutation/🧫️fixtures/🧫️outcome-code/🔣️.json",
    """    { "code": "mutation.clamped", "level": "warning", "meaning": "A value was clamped into its admissible range." },
    { "code": "mutation.duplicate-id", "level": "fatal", "meaning": "The mutation would mint an identity that already exists." },
    { "code": "mutation.invariant", "level": "fatal", "meaning": "The payload violates an invariant its leaf schema states (bounds or x-semio-invariant)." },
""",
    """    { "code": "mutation.clamped", "level": "warning", "meaning": "A value was clamped into its admissible range." },
    { "code": "mutation.precondition-drifted", "level": "warning", "meaning": "The mutation applied, but a precondition it was recorded under no longer holds (state-dependent)." },
    { "code": "mutation.duplicate-id", "level": "fatal", "meaning": "The mutation would mint an identity that already exists." },
    { "code": "mutation.invariant", "level": "fatal", "meaning": "The payload violates an invariant its leaf schema states (bounds or x-semio-invariant)." },
    { "code": "mutation.inverse-refused", "level": "fatal", "meaning": "The mutation cannot be reversed on the state it replays onto, so it is not applied (state-dependent)." },
""",
)
hunk(
    f"{FW}/📡️replication/⚔️conflict/🧬️schema/🔣️replay-report/🔣️.json",
    """            "mutation.clamped",
            "mutation.duplicate-id",
            "mutation.invariant",
            "mutation.cascade"
""",
    """            "mutation.clamped",
            "mutation.precondition-drifted",
            "mutation.duplicate-id",
            "mutation.invariant",
            "mutation.inverse-refused",
            "mutation.cascade"
""",
)
hunk(
    f"{OSM}/🏪️store/🧬️schema/🔣️supersede-replay/🔣️.json",
    '"mutation.clamped", "mutation.duplicate-id", "mutation.invariant", "mutation.cascade"]',
    '"mutation.clamped", "mutation.precondition-drifted", "mutation.duplicate-id", "mutation.invariant", "mutation.inverse-refused", "mutation.cascade"]',
)

# 🦀️ The typed vocabulary and the table persistence validates against.
hunk(
    MUTATION,
    """    NoOp,
    Partial,
    Clamped,
    DuplicateId,
    Invariant,
    Cascade,
}
""",
    """    NoOp,
    Partial,
    Clamped,
    PreconditionDrifted,
    DuplicateId,
    Invariant,
    InverseRefused,
    Cascade,
}
""",
)
hunk(
    MUTATION,
    "    pub const ALL: [OutcomeCode; 9] = [Self::TargetMissing, Self::TargetReferenced, Self::TargetMismatch, Self::NoOp, Self::Partial, Self::Clamped, Self::DuplicateId, Self::Invariant, Self::Cascade];\n",
    "    pub const ALL: [OutcomeCode; 11] = [Self::TargetMissing, Self::TargetReferenced, Self::TargetMismatch, Self::NoOp, Self::Partial, Self::Clamped, Self::PreconditionDrifted, Self::DuplicateId, Self::Invariant, Self::InverseRefused, Self::Cascade];\n",
)
hunk(
    MUTATION,
    """            Self::Clamped => "mutation.clamped",
            Self::DuplicateId => "mutation.duplicate-id",
            Self::Invariant => "mutation.invariant",
""",
    """            Self::Clamped => "mutation.clamped",
            Self::PreconditionDrifted => "mutation.precondition-drifted",
            Self::DuplicateId => "mutation.duplicate-id",
            Self::Invariant => "mutation.invariant",
            Self::InverseRefused => "mutation.inverse-refused",
""",
)
hunk(
    MUTATION,
    """            Self::NoOp | Self::Partial | Self::Clamped => semio_framework_diagnostic::Severity::Warning,
            Self::DuplicateId | Self::Invariant => semio_framework_diagnostic::Severity::Fatal,
""",
    """            Self::NoOp | Self::Partial | Self::Clamped | Self::PreconditionDrifted => semio_framework_diagnostic::Severity::Warning,
            Self::DuplicateId | Self::Invariant | Self::InverseRefused => semio_framework_diagnostic::Severity::Fatal,
""",
)
hunk(
    MUTATION,
    """pub const OUTCOME_CODES: [(&str, semio_framework_diagnostic::Severity); 9] = {
    let mut table = [("", semio_framework_diagnostic::Severity::Info); 9];
""",
    """pub const OUTCOME_CODES: [(&str, semio_framework_diagnostic::Severity); 11] = {
    let mut table = [("", semio_framework_diagnostic::Severity::Info); 11];
""",
)
hunk(
    MUTATION,
    "`code` is one of the frozen nine `mutation.*`\n",
    "`code` is one of the frozen `mutation.*`\n",
)
hunk(
    MUTATION,
    "/// `mutation.target-mismatch` — closed set, no per-plugin codes, ever); `message` is English prose\n",
    "/// `mutation.target-mismatch`, 2026-10-05 by the `Warning` `mutation.precondition-drifted` and the `Fatal`\n/// `mutation.inverse-refused` — closed set, no per-plugin codes, ever); `message` is English prose\n",
)

# 🟦️ The TypeScript twin of the table.
hunk(
    f"{FW}/📡️replication/🟦️.ts",
    """  ["mutation.clamped", "warning"],
  ["mutation.duplicate-id", "fatal"],
  ["mutation.invariant", "fatal"],
""",
    """  ["mutation.clamped", "warning"],
  ["mutation.precondition-drifted", "warning"],
  ["mutation.duplicate-id", "fatal"],
  ["mutation.invariant", "fatal"],
  ["mutation.inverse-refused", "fatal"],
""",
)

# 🗣️ The words: the guest's history body, then the React shell's label keys, their type and both locale bundles.
hunk(
    f"{OSM}/🔌️plugin/⏪️time-travel/🦀️.rs",
    """        "mutation.clamped" => Some(("Clamped", "Begrenzt")),
        "mutation.duplicate-id" => Some(("Duplicate id", "ID bereits vergeben")),
        "mutation.invariant" => Some(("Invalid state", "Ungültiger Zustand")),
""",
    """        "mutation.clamped" => Some(("Clamped", "Begrenzt")),
        "mutation.precondition-drifted" => Some(("Precondition drifted", "Vorbedingung nicht mehr erfüllt")),
        "mutation.duplicate-id" => Some(("Duplicate id", "ID bereits vergeben")),
        "mutation.invariant" => Some(("Invalid state", "Ungültiger Zustand")),
        "mutation.inverse-refused" => Some(("Cannot be reversed", "Nicht umkehrbar")),
""",
)
hunk(
    SHELL,
    "/** ⚖️ Maps one of the frozen nine `mutation.*` codes (contract freeze §C2, extended 2026-09-30 by the two state-dependent\n",
    "/** ⚖️ Maps one of the frozen `mutation.*` codes (contract freeze §C2, extended 2026-09-30 by the two state-dependent\n",
)
hunk(
    SHELL,
    """    case "mutation.clamped":
      return "ui.mutation.code.clamped";
    case "mutation.duplicate-id":
      return "ui.mutation.code.duplicateId";
    case "mutation.invariant":
      return "ui.mutation.code.invariant";
""",
    """    case "mutation.clamped":
      return "ui.mutation.code.clamped";
    case "mutation.precondition-drifted":
      return "ui.mutation.code.preconditionDrifted";
    case "mutation.duplicate-id":
      return "ui.mutation.code.duplicateId";
    case "mutation.invariant":
      return "ui.mutation.code.invariant";
    case "mutation.inverse-refused":
      return "ui.mutation.code.inverseRefused";
""",
)
hunk(
    f"{FW}/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx",
    """        readonly clamped: UiLabelValue;
        readonly duplicateId: UiLabelValue;
        readonly invariant: UiLabelValue;
""",
    """        readonly clamped: UiLabelValue;
        readonly preconditionDrifted: UiLabelValue;
        readonly duplicateId: UiLabelValue;
        readonly invariant: UiLabelValue;
        readonly inverseRefused: UiLabelValue;
""",
)
hunk(
    BUNDLES,
    """            clamped: { label: { normal: "Begrenzt", beginner: "Ein Wert wurde auf den zulässigen Bereich begrenzt." } },
            duplicateId: { label: { normal: "ID bereits vergeben", beginner: "Es existiert bereits ein Element mit dieser ID." } },
            invariant: { label: { normal: "Ungültiger Zustand", beginner: "Diese Änderung würde einen ungültigen Zustand erzeugen." } },
""",
    """            clamped: { label: { normal: "Begrenzt", beginner: "Ein Wert wurde auf den zulässigen Bereich begrenzt." } },
            preconditionDrifted: { label: { normal: "Vorbedingung nicht mehr erfüllt", beginner: "Die Änderung wurde angewendet, aber eine Bedingung, unter der sie aufgezeichnet wurde, gilt nicht mehr." } },
            duplicateId: { label: { normal: "ID bereits vergeben", beginner: "Es existiert bereits ein Element mit dieser ID." } },
            invariant: { label: { normal: "Ungültiger Zustand", beginner: "Diese Änderung würde einen ungültigen Zustand erzeugen." } },
            inverseRefused: { label: { normal: "Nicht umkehrbar", beginner: "Diese Änderung lässt sich auf dem aktuellen Dokument nicht umkehren und wurde deshalb nicht angewendet." } },
""",
)
hunk(
    BUNDLES,
    """            clamped: { label: { normal: "Clamped", beginner: "A value was clamped to its valid range." } },
            duplicateId: { label: { normal: "Duplicate id", beginner: "An element with this id already exists." } },
            invariant: { label: { normal: "Invalid state", beginner: "This change would leave the document in an invalid state." } },
""",
    """            clamped: { label: { normal: "Clamped", beginner: "A value was clamped to its valid range." } },
            preconditionDrifted: { label: { normal: "Precondition drifted", beginner: "The change was applied, but a condition it was recorded under no longer holds." } },
            duplicateId: { label: { normal: "Duplicate id", beginner: "An element with this id already exists." } },
            invariant: { label: { normal: "Invalid state", beginner: "This change would leave the document in an invalid state." } },
            inverseRefused: { label: { normal: "Cannot be reversed", beginner: "This change cannot be reversed on the current document, so it was not applied." } },
""",
)


def main():
    check = "--check" in sys.argv[1:]
    contents, pending = {}, 0
    for path, old, new in HUNKS:
        text = contents.get(path)
        if text is None:
            text = (ROOT / path).read_text(encoding="utf-8")
        if new in text:
            contents[path] = text
            continue
        count = text.count(old)
        if count != 1:
            sys.exit(f"{path}: anchor resolves {count} times, expected 1:\n{old[:200]}")
        contents[path] = text.replace(old, new)
        pending += 1
        if check:
            print(f"pending: {path}: {old.splitlines()[0][:100]}")
    if check:
        print(f"{pending} pending of {len(HUNKS)} hunks in {len(contents)} files")
        return
    for path, text in contents.items():
        if (ROOT / path).read_text(encoding="utf-8") != text:
            (ROOT / path).write_text(text, encoding="utf-8")
    print(f"applied {pending} of {len(HUNKS)} hunks in {len(contents)} files")


if __name__ == "__main__":
    main()
