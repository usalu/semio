# 🔬 Anchored text edit of taxonomy.json registering the architecture-course rename (teaching-course kinds, ventilation topic, concept pets, proctor course module). Idempotent; aborts when an anchor is missing or not unique.
import json, sys
T = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
s = open(T, encoding="utf8").read()

def once(old, new):
    global s
    if s.count(old) != 1:
        sys.exit(f"anchor count {s.count(old)} for: {old[:80]!r}")
    s = s.replace(old, new)

def kind(id_, emoji, slug, parents):
    ps = ",\n".join(f'        "{p}"' for p in parents)
    return f'    "{id_}": {{\n      "emoji": "{emoji}",\n      "slugPattern": "^{slug}$",\n      "allowEmojiOnly": false,\n      "parentKindIds": [\n{ps}\n      ]\n    }}'

once('''    "teaching-quiz": {
      "emoji": "❓️",
      "slugPattern": "^quiz$",
      "allowEmojiOnly": false,
      "parentKindIds": [
        "teaching-architecture",
        "teaching-physics",
        "teaching-heating",
        "teaching-cooling",
        "teaching-demand"
      ]
    },''', kind("teaching-ventilation", "🌬️", "ventilation", ["teaching-energy"]) + ''',
    "teaching-quiz": {
      "emoji": "❓️",
      "slugPattern": "^quiz$",
      "allowEmojiOnly": false,
      "parentKindIds": [
        "teaching-physics",
        "teaching-heating",
        "teaching-cooling",
        "teaching-demand",
        "teaching-ventilation"
      ]
    },''')

old_quiz_children = ",\n".join([
    kind("teaching-quiz-stack", "🧱️", "stack", ["teaching-quiz"]),
    kind("teaching-quiz-catalog", "📚️", "catalog", ["teaching-quiz"]),
    kind("teaching-quiz-e2e", "🎭️", "e2e", ["teaching-quiz"]),
    kind("teaching-quiz-e2e-learner", "🚶️", "learner", ["teaching-quiz-e2e"]),
])
new_course_kinds = ",\n".join([
    kind("teaching-course", "📚️", "course", ["teaching-architecture"]),
    kind("teaching-course-stack", "🧱️", "stack", ["teaching-course"]),
    kind("teaching-course-content", "📚️", "course", ["teaching-course"]),
    kind("teaching-course-e2e", "🎭️", "e2e", ["teaching-course"]),
    kind("teaching-course-e2e-learner", "🚶️", "learner", ["teaching-course-e2e"]),
])
once(old_quiz_children, new_course_kinds)

once('''      "memberNames": [
        "❓️quiz",
        "⚡️energy"
      ],''', '''      "memberNames": [
        "📚️course",
        "⚡️energy"
      ],''')
once('''      "memberNames": [
        "🧲️physics",
        "🔥️heating",
        "❄️cooling",
        "📊️demand"
      ],''', '''      "memberNames": [
        "🧲️physics",
        "🔥️heating",
        "❄️cooling",
        "📊️demand",
        "🌬️ventilation"
      ],''')
once('''      "ownerKindIds": [
        "teaching-physics",
        "teaching-heating",
        "teaching-cooling",
        "teaching-demand"
      ],
      "memberNames": [
        "❓️quiz"
      ],''', '''      "ownerKindIds": [
        "teaching-physics",
        "teaching-heating",
        "teaching-cooling",
        "teaching-demand",
        "teaching-ventilation"
      ],
      "memberNames": [
        "❓️quiz"
      ],''')

course_members = ["🔒️sessions", "🎚️mode", "⚖️readiness", "🌐️site", "🎁️stack-bundle", "🏷️release", "🐳️docker-image", "📦️image", "🧱️docker-stack", "🩺️docker-image-check",
  "🌐️publication", "🌬️ventilation", "🎛️dashboard", "🎚️parameters", "🎭️e2e-output", "🏋️capacity", "🐳️context", "🐳️runtime", "🐾️context-casts", "📚️course", "🔌️port", "🗂️topics", "🪁️runtime", "🪂️canopies", "🪢️presence", "🪧️logo", "🚀️readiness", "🚛️bundle", "🛑️cancellation"]
names = ",\n".join(f'        "{n}"' for n in course_members)
anchor = '''    "members-of-teaching-pets": {'''
once(anchor, f'''    "members-of-teaching-course": {{
      "ownerKindIds": [
        "teaching-course"
      ],
      "memberNames": [
{names}
      ],
      "source": "registry"
    }},
{anchor}''')

pets = ["ƒ️formula", "∞️continuous-parameter", "〰️double-curved-surface", "🎚️slider", "🎨️colour-wheel", "🏞️minimized-surface", "📄️worksheet", "📉️pareto", "📊️bar-chart", "📋️execution-plan", "📐️plan-2d", "📚️workbook", "🔢️discrete-parameter", "🔲️cell", "🦏️rhino", "🦗️grasshopper", "🧊️model-3d", "🧮️quantities", "🟧️heatmap"]
once('''        "🌡️thermy",
        "🖥️servy"
      ],''', '''        "🌡️thermy",
        "🖥️servy",
''' + ",\n".join(f'        "{n}"' for n in pets) + '''
      ],''')

once('''        "📚️compiler",
        "📚️library",''', '''        "📚️compiler",
        "📚️course",
        "📚️library",''')
open(T, "w", encoding="utf8").write(s)
print("ok")
