"""🧪️ W2-W-norm-3 follow-up: renames the four EN 1998 kinds whose leaf directories left no room for a vector under
the 240-byte canonical-pair path budget (coordinator decision, 2026-09-30).

change-system-base-shear-resistance-n → change-system-v-rd-n   (V_Rd, as `change-bridge-v-rd-n`)
change-building-elevation-regular     → change-elevation-regular (EN 1998-1 §4.2.3.3 regularity in elevation)
change-member-detailing-compatible    → change-member-detailing  (detailing for local ductility consistent with q)
change-building-masonry-wall-area-ratio → change-masonry-wall-ratio (EN 1998-1 §9.7.2 wall area ratio p_A)

Every spelling moves at once — leaf and fixture directories, kind, variant, camelCase wire tag, module, binary tag
constant, descriptor entity and display name, labels — over every text file of the EN 1998 artifact. No alias remains.
"""
import os

ARTIFACT = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"
SUBSET = f"{ARTIFACT}/🏅️standards/🔖️1/🪆️subsets/✳️any"
RENAMES = [
    # old dir, new dir, old kind, new kind, old variant, new variant, old entity, new entity
    ("💪️change-system-base-shear-resistance-n", "💪️change-system-v-rd-n", "change-system-base-shear-resistance-n", "change-system-v-rd-n", "ChangeSystemBaseShearResistanceN", "ChangeSystemVRdN", "system-base-shear-resistance-n", "system-v-rd-n"),
    ("📏️change-building-elevation-regular", "📏️change-elevation-regular", "change-building-elevation-regular", "change-elevation-regular", "ChangeBuildingElevationRegular", "ChangeElevationRegular", "building-elevation-regular", "elevation-regular"),
    ("✅️change-member-detailing-compatible", "✅️change-member-detailing", "change-member-detailing-compatible", "change-member-detailing", "ChangeMemberDetailingCompatible", "ChangeMemberDetailing", "member-detailing-compatible", "member-detailing"),
    ("🧱change-building-masonry-wall-area-ratio", "🧱️change-masonry-wall-ratio", "change-building-masonry-wall-area-ratio", "change-masonry-wall-ratio", "ChangeBuildingMasonryWallAreaRatio", "ChangeMasonryWallRatio", "building-masonry-wall-area-ratio", "masonry-wall-ratio"),
]
LABELS = [
    ('"displayName": "Change system base shear resistance"', '"displayName": "Change system shear resistance V_Rd"'),
    ('protocol::LocalizedLabel::native("Change base shear resistance of the system", "Widerstand gegen die Gesamterdbebenkraft des Systems ändern")',
     'protocol::LocalizedLabel::native("Change shear resistance V_Rd of the system", "Querkraftwiderstand V_Rd des Systems ändern")'),
    ('"displayName": "Change building elevation regularity"', '"displayName": "Change regularity in elevation"'),
    ('"displayName": "Change member detailing compatibility"', '"displayName": "Change member detailing conformity"'),
    ('"emoji": "🧱"', '"emoji": "🧱️"'),
]


def camel(pascal):
    return pascal[0].lower() + pascal[1:]


def pairs():
    out = []
    for old_dir, new_dir, old_kind, new_kind, old_variant, new_variant, old_entity, new_entity in RENAMES:
        out += [
            (old_dir, new_dir),
            (old_kind, new_kind),
            (old_variant, new_variant),
            (camel(old_variant), camel(new_variant)),
            (old_kind.replace("-", "_"), new_kind.replace("-", "_")),
            ("TAG_" + old_kind.replace("-", "_").upper(), "TAG_" + new_kind.replace("-", "_").upper()),
            (f'entity: "{old_entity}"', f'entity: "{new_entity}"'),
        ]
    return out


def rewrite():
    changed = 0
    for root, dirs, files in os.walk(ARTIFACT):
        dirs[:] = [name for name in dirs if name not in ("dist", "target", "node_modules")]
        for name in files:
            if not name.endswith((".rs", ".ts", ".json", ".py", ".feature", ".semio", ".graphql", ".proto", ".md", ".g4", ".ebnf")):
                continue
            path = os.path.join(root, name)
            try:
                text = open(path, encoding="utf-8").read()
            except UnicodeDecodeError:
                continue
            new = text
            for old, replacement in pairs() + LABELS:
                new = new.replace(old, replacement)
            if new != text:
                open(path, "w", encoding="utf-8").write(new)
                changed += 1
    for old_dir, new_dir, *_ in RENAMES:
        for parent in (f"{SUBSET}/🧬️schema/🧬️mutations", f"{SUBSET}/🧫️fixtures/🧬️mutations"):
            if os.path.isdir(f"{parent}/{old_dir}"):
                os.rename(f"{parent}/{old_dir}", f"{parent}/{new_dir}")
    print("rewrote", changed, "files")


if __name__ == "__main__":
    rewrite()
