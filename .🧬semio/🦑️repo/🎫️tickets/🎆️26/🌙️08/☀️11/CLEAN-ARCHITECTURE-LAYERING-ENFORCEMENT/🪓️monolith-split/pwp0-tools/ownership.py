# 🗂️ File ownership of the plugin tree by work package: the "owns" column of PLAN-PLUGIN-CRATES.md 12.1 as longest-prefix rules (paths relative to PLUG/).
OWNER = {
 1: ['🚪️io/', '🧬️schema/', '🌐host/', '📡️backbone/', '⚛️reactor/🧵️executor/', '🪪️identity/', '🕹️interaction/', '♻️cancellation/', '⏳️operation-progress/', '🌱️initialization/', '📬️completion/', '🪟️window/', '🪟️window-kits/', '🖱️context-menu/'],
 2: ['📜️declaration/', '🏗️builder/🧩️app/', '🏗️builder/🔌️program/', '⚛️reactor/🪟️surfaces/', '🫧️transient/', '🎚️config/', '📝️draft/', '👥️presence/', '🧵️task/'],
 3: ['🪆️child/', '🧩️composition/', '🗃️document/', '🎞️media/', '👷️job/', '🎮️command/', '👥️peer/', '🧵️retained-command/', '🧩️extension/', '💡️inference/', '🛂️describe/'],
 4: ['⚛️reactor/', '🚪️lifetime/'],
 5: ['⏯️tool-run/', '🛠️tool-machine/', '⏪️time-travel/'],
 6: ['🧵️instance/🏪️stores/', '🧵️instance/📜️history/', '🧵️instance/🪆️children/', '🧵️instance/👷️jobs/', '🧵️instance/📄️docs/', '🧵️instance/🧩️extensions/'],
 7: ['⏯️tool-run/🧵️instance/', '⏪️time-travel/🧵️instance/', '🧵️instance/', '🪜️', '♻️lifecycle/', '🏗️builder/', '🧪️testing/', '🧪️tests/', '⚖️authoring-law/', '🦀️.rs', '📦️packages/', '🧷️companions/', '🪝️mounts/'],
}
# instance-family files that live in lower module directories but belong to the instance family (7.4 and 7.9)
INSTANCE_FILES = {'🎮️command/🎟️lease/': 7, '🎮️command/📥️ingress/': 6, '🪆️child/👁️read/🧪️test-view/': 6, '🪟️window/🫧️transient/🔁️document-replacement/': 7}
RULES = sorted([(p, wp) for wp, ps in OWNER.items() for p in ps] + list(INSTANCE_FILES.items()), key=lambda x: -len(x[0]))
def owner_wp(f):
    for p, wp in RULES:
        if f.startswith(p) or f == p: return wp
    return None
