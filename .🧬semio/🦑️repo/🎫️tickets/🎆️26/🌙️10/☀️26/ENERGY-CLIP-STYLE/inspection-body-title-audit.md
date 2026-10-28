# Persistent Video Header Removal: Findings

The current shared scene_title creates a real persistent primary-outlined video/chapter-name chip. beat_subtitle depends on that object via next_to(title,DOWN) and align_to(title,LEFT); deleting only the visible title without changing that API would leave an obsolete layout dependency. The exact canonical call inventory follows below.

## Call Patterns And Scope

Across 38 canonical authored scripts plus the BIM/optimization shared scene owners,31 modules contain105 scene_title calls,23 play_scene_title calls,104 beat_subtitle calls and5 calculation _heading calls. Most Energy scenes assign title=scene_title(TITLE_DE), then either play_scene_title(self,title) or self.add(title), followed by beat_subtitle(instruction,title). Later section subtitles (micro_sub and density_sub) reuse that same anchor.

Exceptions requiring semantic care:

- Heating calculation _heading(text) wraps scene_title for five instructional headings such as heat-loss overview and annual energy. Those headings are actual teaching content and must survive as standalone section headings. UltimateEnergyBalance also directly creates a content-specific main-equation heading with scene_title; retain its instructional text through the new heading API. Its nearby descriptive subtitle is explicitly next_to(title,DOWN), so that genuine instructional anchor remains meaningful.
- BIM ClipScene.present adds _fonts.scene_title(self.TITLE[language]) persistently. Remove this scene-name header. Information creates a Villa Savoye/ground-floor beat subtitle using a freshly constructed scene_title only as an anchor; objects similarly constructs subtitle anchors inside its _heading helper. Replace those calls with actual standalone instructional headings.
- Optimization ClipScene.present stores/adds self._title, and clear_stage excludes it. Remove the field and exclusion rather than retaining an empty sentinel. Extrema._sub anchors instructional subtitles to self._title, and its later keep set references self._title; migrate these references while retaining the instructional subtitle itself.
- Plot axis titles, Winter/Sommer labels, equipment-panel titles, source/standard notes and technical diagram labels are not persistent video headers and must remain. The line inventory includes such nearby title-named variables for reviewer visibility; do not mechanically delete every identifier containing title.

## Minimal Clean API Proposal

Replace the two-tier API with a single real section_heading(text) function positioned directly at the shared top-left instructional slot using explicit layout constants. It creates the existing styled instructional chip with no title-anchor argument. Migrate beat_subtitle call sites to section_heading and delete scene_title/play_scene_title and obsolete imports, variables, add/play calls and lifecycle keep-set references. Use ordinary existing FadeIn/Write at call sites where an instructional entrance animation is required. Calculation instructional headings use section_heading too; actual descriptions can still position relative to that real content heading. No transparent object, dummy anchor, masking, cropping, alias or compatibility branch is needed.

Move retained instructional headings into the first top-left row and validate their bounds against notes/content; existing note-band assumptions mention the old second-row subtitle position and need a focused layout review. Caption timings/narration remain independent; remove only the old header entrance animation budget where no instructional animation replaces it. Font helper changes intentionally invalidate every canonical body source and require clean production guards/rerender, not metadata rewriting. Portable fixtures should distinguish forbidden persistent scene-name headers from retained section/axis labels and validate real rendered bounds against the existing Manim/Pillow oracle.

---

# Persistent Video Header Inventory

Read-only AST inventory of 40 canonical authored/shared scene modules. No production edit or rendering was performed.

## Counts

{'scene_title': 105, 'play_scene_title': 23, 'beat_subtitle': 104, '_heading': 5}
31 modules contain the relevant calls.

## Exact Calls And Title Dependencies

### 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py
- L206: `scene_title`: `scene_title(TITLE_DE)`
- L207: `play_scene_title`: `play_scene_title(self, title)`
- L208: `beat_subtitle`: `beat_subtitle('Direkte Sonnenstrahlung', title)`
- L430: `scene_title`: `scene_title(TITLE_DE)`
- L432: `beat_subtitle`: `beat_subtitle('Solare Kühllast', title)`
Title-related source lines:
- L23: `scene_title = _manim_module.scene_title`
- L24: `play_scene_title = _manim_module.play_scene_title`
- L26: `beat_subtitle = _manim_module.beat_subtitle`
- L206: `title = scene_title(TITLE_DE)`
- L207: `play_scene_title(self, title)`
- L208: `subtitle = beat_subtitle("Direkte Sonnenstrahlung", title)`
- L210: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L430: `title = scene_title(TITLE_DE)`
- L431: `self.add(title)`
- L432: `subtitle = beat_subtitle("Solare Kühllast", title)`
- L434: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py
- L257: `scene_title`: `scene_title(TITLE_DE)`
- L258: `play_scene_title`: `play_scene_title(self, title)`
- L259: `beat_subtitle`: `beat_subtitle('Im Winter: Erwünschte kostenlose Wärme', title)`
- L327: `scene_title`: `scene_title(TITLE_DE)`
- L329: `beat_subtitle`: `beat_subtitle('Im Sommer: Überhitzung hinter Glas', title)`
- L399: `scene_title`: `scene_title(TITLE_DE)`
- L401: `beat_subtitle`: `beat_subtitle('Die Kühllast als abzuführende Leistung', title)`
Title-related source lines:
- L19: `scene_title = _manim_module.scene_title`
- L20: `play_scene_title = _manim_module.play_scene_title`
- L22: `beat_subtitle = _manim_module.beat_subtitle`
- L172: `title = Text("Raumtemperatur", font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(th["group"], UP, buff=0.18)`
- L180: `return {"static": VGroup(th["group"], title), "column": th["column"], "read": read}`
- L257: `title = scene_title(TITLE_DE)`
- L258: `play_scene_title(self, title)`
- L259: `subtitle = beat_subtitle("Im Winter: Erwünschte kostenlose Wärme", title)`
- L261: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L327: `title = scene_title(TITLE_DE)`
- L328: `self.add(title)`
- L329: `subtitle = beat_subtitle("Im Sommer: Überhitzung hinter Glas", title)`
- L331: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L399: `title = scene_title(TITLE_DE)`
- L400: `self.add(title)`
- L401: `subtitle = beat_subtitle("Die Kühllast als abzuführende Leistung", title)`
- L403: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py
- L150: `scene_title`: `scene_title(TITLE_DE)`
- L151: `play_scene_title`: `play_scene_title(self, title)`
- L152: `beat_subtitle`: `beat_subtitle('Erst die Last senken, dann lüften', title)`
- L290: `scene_title`: `scene_title(TITLE_DE)`
- L292: `beat_subtitle`: `beat_subtitle('Sorptionsgestützte Kühlung (DEC)', title)`
- L504: `scene_title`: `scene_title(TITLE_DE)`
- L506: `beat_subtitle`: `beat_subtitle('Strategie: freie Kühlung zuerst', title)`
Title-related source lines:
- L22: `scene_title = _manim_module.scene_title`
- L23: `play_scene_title = _manim_module.play_scene_title`
- L25: `beat_subtitle = _manim_module.beat_subtitle`
- L150: `title = scene_title(TITLE_DE)`
- L151: `play_scene_title(self, title)`
- L152: `subtitle = beat_subtitle("Erst die Last senken, dann lüften", title)`
- L154: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L290: `title = scene_title(TITLE_DE)`
- L291: `self.add(title)`
- L292: `subtitle = beat_subtitle("Sorptionsgestützte Kühlung (DEC)", title)`
- L294: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L504: `title = scene_title(TITLE_DE)`
- L505: `self.add(title)`
- L506: `subtitle = beat_subtitle("Strategie: freie Kühlung zuerst", title)`
- L508: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py
- L182: `scene_title`: `scene_title(TITLE_DE)`
- L183: `play_scene_title`: `play_scene_title(self, title)`
- L184: `beat_subtitle`: `beat_subtitle('Die umschlossene Umgebung', title)`
- L265: `scene_title`: `scene_title(TITLE_DE)`
- L267: `beat_subtitle`: `beat_subtitle('Menschliche Stoffwechselwärme', title)`
- L595: `scene_title`: `scene_title(TITLE_DE)`
- L597: `beat_subtitle`: `beat_subtitle('Geräte, Steckerlasten und Beleuchtung', title)`
- L821: `scene_title`: `scene_title(TITLE_DE)`
- L823: `beat_subtitle`: `beat_subtitle('Summe der internen Gewinne', title)`
- L936: `scene_title`: `scene_title(TITLE_DE)`
- L938: `beat_subtitle`: `beat_subtitle('Minderung & intelligentes Design', title)`
Title-related source lines:
- L25: `scene_title = _manim_module.scene_title`
- L26: `play_scene_title = _manim_module.play_scene_title`
- L28: `beat_subtitle = _manim_module.beat_subtitle`
- L182: `title = scene_title(TITLE_DE)`
- L183: `play_scene_title(self, title)`
- L184: `subtitle = beat_subtitle("Die umschlossene Umgebung", title)`
- L186: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L265: `title = scene_title(TITLE_DE)`
- L266: `self.add(title)`
- L267: `subtitle = beat_subtitle("Menschliche Stoffwechselwärme", title)`
- L269: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L595: `title = scene_title(TITLE_DE)`
- L596: `self.add(title)`
- L597: `subtitle = beat_subtitle("Geräte, Steckerlasten und Beleuchtung", title)`
- L599: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L821: `title = scene_title(TITLE_DE)`
- L822: `self.add(title)`
- L823: `subtitle = beat_subtitle("Summe der internen Gewinne", title)`
- L825: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L936: `title = scene_title(TITLE_DE)`
- L937: `self.add(title)`
- L938: `subtitle = beat_subtitle("Minderung & intelligentes Design", title)`
- L940: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1002: `ctrl_title = Text("Smarte Laststeuerung", font_size=LABEL_FONT_SIZE, color=C_COOL)`
- L1003: `ctrl_title.move_to(ctrl_panel.get_top() + DOWN * 0.18)`
- L1055: `FadeIn(ctrl_title),`

### 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py
- L232: `scene_title`: `scene_title(TITLE_OPAQUE_DE)`
- L233: `play_scene_title`: `play_scene_title(self, title)`
- L234: `beat_subtitle`: `beat_subtitle('Opake Bauteile unter Sommerstrahlung', title)`
- L385: `scene_title`: `scene_title(TITLE_LAST_DE)`
- L386: `play_scene_title`: `play_scene_title(self, title)`
- L387: `beat_subtitle`: `beat_subtitle('Die Luft als zweite Last', title)`
Title-related source lines:
- L22: `scene_title = _manim_module.scene_title`
- L23: `play_scene_title = _manim_module.play_scene_title`
- L25: `beat_subtitle = _manim_module.beat_subtitle`
- L232: `title = scene_title(TITLE_OPAQUE_DE)`
- L233: `play_scene_title(self, title)`
- L234: `subtitle = beat_subtitle("Opake Bauteile unter Sommerstrahlung", title)`
- L236: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L385: `title = scene_title(TITLE_LAST_DE)`
- L386: `play_scene_title(self, title)`
- L387: `subtitle = beat_subtitle("Die Luft als zweite Last", title)`
- L389: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py
- L228: `scene_title`: `scene_title(TITLE_DE)`
- L230: `beat_subtitle`: `beat_subtitle('Konvektive Kühlleistung der Zuluft', title)`
- L405: `scene_title`: `scene_title(TITLE_DE)`
- L407: `beat_subtitle`: `beat_subtitle('Gleichgewicht: Kühlleistung = Kühllast', title)`
Title-related source lines:
- L22: `scene_title = _manim_module.scene_title`
- L23: `beat_subtitle = _manim_module.beat_subtitle`
- L228: `title = scene_title(TITLE_DE)`
- L229: `self.add(title)`
- L230: `subtitle = beat_subtitle("Konvektive Kühlleistung der Zuluft", title)`
- L232: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L405: `title = scene_title(TITLE_DE)`
- L406: `self.add(title)`
- L407: `subtitle = beat_subtitle("Gleichgewicht: Kühlleistung = Kühllast", title)`
- L409: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py
- L257: `scene_title`: `scene_title(TITLE_DE)`
- L258: `play_scene_title`: `play_scene_title(self, title)`
- L259: `beat_subtitle`: `beat_subtitle('Drei mechanische Grundtypen', title)`
- L437: `scene_title`: `scene_title(TITLE_DE)`
- L439: `beat_subtitle`: `beat_subtitle('Wärmerückgewinnung im Plattenübertrager', title)`
- L713: `scene_title`: `scene_title(TITLE_DE)`
- L715: `beat_subtitle`: `beat_subtitle('Lüftungssysteme im Vergleich', title)`
- L922: `scene_title`: `scene_title(TITLE_DE)`
- L924: `beat_subtitle`: `beat_subtitle('Vom Luftstrom zum Kanalquerschnitt', title)`
Title-related source lines:
- L34: `scene_title = _manim_module.scene_title`
- L35: `play_scene_title = _manim_module.play_scene_title`
- L37: `beat_subtitle = _manim_module.beat_subtitle`
- L113: `that never competes with the diagram. The chapter title is wide, so the`
- L114: `footnote sits one line lower, level with the beat subtitle.`
- L218: `name = Text(title, font_size=LABEL_FONT_SIZE, color=color).move_to(box.get_center() + UP * 0.24)`
- L257: `title = scene_title(TITLE_DE)`
- L258: `play_scene_title(self, title)`
- L259: `subtitle = beat_subtitle("Drei mechanische Grundtypen", title)`
- L261: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L437: `title = scene_title(TITLE_DE)`
- L438: `self.add(title)`
- L439: `subtitle = beat_subtitle("Wärmerückgewinnung im Plattenübertrager", title)`
- L441: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L713: `title = scene_title(TITLE_DE)`
- L714: `self.add(title)`
- L715: `subtitle = beat_subtitle("Lüftungssysteme im Vergleich", title)`
- L717: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L922: `title = scene_title(TITLE_DE)`
- L923: `self.add(title)`
- L924: `subtitle = beat_subtitle("Vom Luftstrom zum Kanalquerschnitt", title)`
- L926: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/❓️need/🐍️.py
- L253: `scene_title`: `scene_title(TITLE_DE)`
- L254: `play_scene_title`: `play_scene_title(self, title)`
- L255: `beat_subtitle`: `beat_subtitle('Was Lüftung leisten muss', title)`
- L445: `scene_title`: `scene_title(TITLE_DE)`
- L447: `beat_subtitle`: `beat_subtitle('Der Lüftungsweg durchs Haus', title)`
- L614: `scene_title`: `scene_title(TITLE_DE)`
- L616: `beat_subtitle`: `beat_subtitle('Der Fahrplan des Videos', title)`
Title-related source lines:
- L29: `scene_title = _manim_module.scene_title`
- L30: `play_scene_title = _manim_module.play_scene_title`
- L31: `beat_subtitle = _manim_module.beat_subtitle`
- L253: `title = scene_title(TITLE_DE)`
- L254: `play_scene_title(self, title)`
- L255: `subtitle = beat_subtitle("Was Lüftung leisten muss", title)`
- L257: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L445: `title = scene_title(TITLE_DE)`
- L446: `self.add(title)`
- L447: `subtitle = beat_subtitle("Der Lüftungsweg durchs Haus", title)`
- L449: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L614: `title = scene_title(TITLE_DE)`
- L615: `self.add(title)`
- L616: `subtitle = beat_subtitle("Der Fahrplan des Videos", title)`
- L618: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py
- L294: `scene_title`: `scene_title(TITLE_DE)`
- L295: `play_scene_title`: `play_scene_title(self, title)`
- L296: `beat_subtitle`: `beat_subtitle('Wärmekapazität der Luft', title)`
- L463: `scene_title`: `scene_title(TITLE_DE)`
- L465: `beat_subtitle`: `beat_subtitle('Sensible Wärme', title)`
- L620: `scene_title`: `scene_title(TITLE_DE)`
- L622: `beat_subtitle`: `beat_subtitle('Latente Last durch Feuchte', title)`
- L793: `scene_title`: `scene_title(TITLE_DE)`
- L795: `beat_subtitle`: `beat_subtitle('Dieselbe Formel, zwei Jahreszeiten', title)`
Title-related source lines:
- L34: `scene_title = _manim_module.scene_title`
- L35: `play_scene_title = _manim_module.play_scene_title`
- L37: `beat_subtitle = _manim_module.beat_subtitle`
- L294: `title = scene_title(TITLE_DE)`
- L295: `play_scene_title(self, title)`
- L296: `subtitle = beat_subtitle("Wärmekapazität der Luft", title)`
- L298: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L463: `title = scene_title(TITLE_DE)`
- L464: `self.add(title)`
- L465: `subtitle = beat_subtitle("Sensible Wärme", title)`
- L467: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L620: `title = scene_title(TITLE_DE)`
- L621: `self.add(title)`
- L622: `subtitle = beat_subtitle("Latente Last durch Feuchte", title)`
- L624: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L793: `title = scene_title(TITLE_DE)`
- L794: `self.add(title)`
- L795: `subtitle = beat_subtitle("Dieselbe Formel, zwei Jahreszeiten", title)`
- L797: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/📦️volume/🐍️.py
- L502: `scene_title`: `scene_title(TITLE_DE)`
- L504: `beat_subtitle`: `beat_subtitle('Bruttovolumen und Nettovolumen', title)`
- L753: `scene_title`: `scene_title(TITLE_DE)`
- L755: `beat_subtitle`: `beat_subtitle('Luftwechselrate n', title)`
- L956: `scene_title`: `scene_title(TITLE_DE)`
- L958: `beat_subtitle`: `beat_subtitle('Volumenstrom: ein Luftstrom, zwei Schreibweisen', title)`
Title-related source lines:
- L34: `scene_title = _manim_module.scene_title`
- L35: `play_scene_title = _manim_module.play_scene_title`
- L36: `beat_subtitle = _manim_module.beat_subtitle`
- L367: `title = math_label(r"n\;[\mathrm{h^{-1}}]", np.array([x + 0.1, y_hi + 0.22, 0]), size=LABEL_FONT_SIZE,`
- L369: `return {"y_of": y_of, "x": x, "group": VGroup(axis, stub, brk, ticks, labels, title)}`
- L502: `title = scene_title(TITLE_DE)`
- L503: `self.add(title)`
- L504: `subtitle = beat_subtitle("Bruttovolumen und Nettovolumen", title)`
- L506: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L753: `title = scene_title(TITLE_DE)`
- L754: `self.add(title)`
- L755: `subtitle = beat_subtitle("Luftwechselrate n", title)`
- L757: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L956: `title = scene_title(TITLE_DE)`
- L957: `self.add(title)`
- L958: `subtitle = beat_subtitle("Volumenstrom: ein Luftstrom, zwei Schreibweisen", title)`
- L960: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py
- L251: `scene_title`: `scene_title(TITLE_DE)`
- L252: `play_scene_title`: `play_scene_title(self, title)`
- L253: `beat_subtitle`: `beat_subtitle('Sieben Größen, eine Kette', title)`
- L411: `scene_title`: `scene_title(TITLE_DE)`
- L413: `beat_subtitle`: `beat_subtitle('Was Heizung und Kühlung übernehmen', title)`
Title-related source lines:
- L32: `scene_title = _manim_module.scene_title`
- L33: `play_scene_title = _manim_module.play_scene_title`
- L34: `beat_subtitle = _manim_module.beat_subtitle`
- L251: `title = scene_title(TITLE_DE)`
- L252: `play_scene_title(self, title)`
- L253: `subtitle = beat_subtitle("Sieben Größen, eine Kette", title)`
- L255: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L411: `title = scene_title(TITLE_DE)`
- L412: `self.add(title)`
- L413: `subtitle = beat_subtitle("Was Heizung und Kühlung übernehmen", title)`
- L415: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py
- L338: `scene_title`: `scene_title(TITLE_DE)`
- L339: `play_scene_title`: `play_scene_title(self, title)`
- L340: `beat_subtitle`: `beat_subtitle('Das Fenster ist ein Regler', title)`
- L583: `scene_title`: `scene_title(TITLE_DE)`
- L585: `beat_subtitle`: `beat_subtitle('Stoßlüften oder Kipplüftung', title)`
- L820: `scene_title`: `scene_title(TITLE_DE)`
- L822: `beat_subtitle`: `beat_subtitle('Querlüftung: beide Öffnungen zählen', title)`
- L1021: `scene_title`: `scene_title(TITLE_DE)`
- L1023: `beat_subtitle`: `beat_subtitle('Auftrieb: Höhe erzeugt Druck', title)`
- L1251: `scene_title`: `scene_title(TITLE_DE)`
- L1253: `beat_subtitle`: `beat_subtitle('Nachtlüftung: den Speicher entladen', title)`
- L1463: `scene_title`: `scene_title(TITLE_DE)`
- L1465: `beat_subtitle`: `beat_subtitle('Grenzen der freien Lüftung', title)`
Title-related source lines:
- L29: `scene_title = _manim_module.scene_title`
- L30: `play_scene_title = _manim_module.play_scene_title`
- L32: `beat_subtitle = _manim_module.beat_subtitle`
- L146: `def _badge(title, value, color=P_PRIMARY):`
- L161: `Text(title, font_size=LABEL_FONT_SIZE, color=color),`
- L239: `head = math_label(title, size=BODY_FONT_SIZE, color=color)`
- L338: `title = scene_title(TITLE_DE)`
- L339: `play_scene_title(self, title)`
- L340: `subtitle = beat_subtitle("Das Fenster ist ein Regler", title)`
- L342: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L583: `title = scene_title(TITLE_DE)`
- L584: `self.add(title)`
- L585: `subtitle = beat_subtitle("Stoßlüften oder Kipplüftung", title)`
- L587: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L820: `title = scene_title(TITLE_DE)`
- L821: `self.add(title)`
- L822: `subtitle = beat_subtitle("Querlüftung: beide Öffnungen zählen", title)`
- L824: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1021: `title = scene_title(TITLE_DE)`
- L1022: `self.add(title)`
- L1023: `subtitle = beat_subtitle("Auftrieb: Höhe erzeugt Druck", title)`
- L1025: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1251: `title = scene_title(TITLE_DE)`
- L1252: `self.add(title)`
- L1253: `subtitle = beat_subtitle("Nachtlüftung: den Speicher entladen", title)`
- L1255: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1463: `title = scene_title(TITLE_DE)`
- L1464: `self.add(title)`
- L1465: `subtitle = beat_subtitle("Grenzen der freien Lüftung", title)`
- L1467: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/♨️system-losses/🐍️.py
- L614: `scene_title`: `scene_title(TITLE_DE)`
- L616: `beat_subtitle`: `beat_subtitle('Übergabe, Verteilung, Speicherung', title)`
- L801: `scene_title`: `scene_title(TITLE_DE)`
- L803: `beat_subtitle`: `beat_subtitle('Erzeugung: Wirkungsgrad oder Arbeitszahl', title)`
- L1026: `scene_title`: `scene_title(TITLE_DE)`
- L1028: `beat_subtitle`: `beat_subtitle('Die Anlagenaufwandszahl e_p', title)`
Title-related source lines:
- L22: `scene_title = _manim_module.scene_title`
- L23: `play_scene_title = _manim_module.play_scene_title`
- L25: `beat_subtitle = _manim_module.beat_subtitle`
- L614: `title = scene_title(TITLE_DE)`
- L615: `self.add(title)`
- L616: `subtitle = beat_subtitle("Übergabe, Verteilung, Speicherung", title)`
- L618: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L721: `keep = (title, subtitle, din, caption)`
- L801: `title = scene_title(TITLE_DE)`
- L802: `self.add(title)`
- L803: `subtitle = beat_subtitle("Erzeugung: Wirkungsgrad oder Arbeitszahl", title)`
- L805: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1026: `title = scene_title(TITLE_DE)`
- L1027: `self.add(title)`
- L1028: `subtitle = beat_subtitle("Die Anlagenaufwandszahl e_p", title)`
- L1030: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1089: `keep = (title, subtitle, din, caption)`

### 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py
- L584: `scene_title`: `scene_title(TITLE_DE)`
- L585: `play_scene_title`: `play_scene_title(self, title)`
- L586: `beat_subtitle`: `beat_subtitle('Vom Gesetz zur Rechenmethode', title)`
- L718: `scene_title`: `scene_title(TITLE_DE)`
- L720: `beat_subtitle`: `beat_subtitle('Warum Monat für Monat bilanziert wird', title)`
- L953: `scene_title`: `scene_title(TITLE_DE)`
- L955: `beat_subtitle`: `beat_subtitle('Klimadaten: das Testreferenzjahr', title)`
- L1142: `scene_title`: `scene_title(TITLE_DE)`
- L1144: `beat_subtitle`: `beat_subtitle('Heizgrenze & Gradtagszahl', title)`
Title-related source lines:
- L22: `scene_title = _manim_module.scene_title`
- L23: `play_scene_title = _manim_module.play_scene_title`
- L25: `beat_subtitle = _manim_module.beat_subtitle`
- L584: `title = scene_title(TITLE_DE)`
- L585: `play_scene_title(self, title)`
- L586: `subtitle = beat_subtitle("Vom Gesetz zur Rechenmethode", title)`
- L588: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L661: `_wrap_up(self, keep=[title, subtitle, din], run_time=0.6)`
- L718: `title = scene_title(TITLE_DE)`
- L719: `self.add(title)`
- L720: `subtitle = beat_subtitle("Warum Monat für Monat bilanziert wird", title)`
- L722: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L895: `_wrap_up(self, keep=[title, subtitle, din])`
- L953: `title = scene_title(TITLE_DE)`
- L954: `self.add(title)`
- L955: `subtitle = beat_subtitle("Klimadaten: das Testreferenzjahr", title)`
- L957: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1076: `_wrap_up(self, keep=[title, subtitle, din])`
- L1142: `title = scene_title(TITLE_DE)`
- L1143: `self.add(title)`
- L1144: `subtitle = beat_subtitle("Heizgrenze & Gradtagszahl", title)`
- L1146: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1262: `_wrap_up(self, keep=[title, subtitle, din])`

### 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚡️demand-power/🐍️.py
- L417: `scene_title`: `scene_title(TITLE_DE)`
- L418: `play_scene_title`: `play_scene_title(self, title)`
- L419: `beat_subtitle`: `beat_subtitle('Leistung ist eine Rate, Energie eine Menge', title)`
- L615: `scene_title`: `scene_title(TITLE_DE)`
- L617: `beat_subtitle`: `beat_subtitle('Die Norm-Heizlast nach DIN EN 12831-1', title)`
- L877: `scene_title`: `scene_title(TITLE_DE)`
- L879: `beat_subtitle`: `beat_subtitle('Die Jahresdauerlinie', title)`
- L1072: `scene_title`: `scene_title(TITLE_DE)`
- L1074: `beat_subtitle`: `beat_subtitle('Auslegen auf die Spitze, nicht auf den Mittelwert', title)`
Title-related source lines:
- L21: `scene_title = _manim_module.scene_title`
- L22: `play_scene_title = _manim_module.play_scene_title`
- L24: `beat_subtitle = _manim_module.beat_subtitle`
- L417: `title = scene_title(TITLE_DE)`
- L418: `play_scene_title(self, title)`
- L419: `subtitle = beat_subtitle("Leistung ist eine Rate, Energie eine Menge", title)`
- L421: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L544: `_clear_stage(self, (title, subtitle, din))`
- L615: `title = scene_title(TITLE_DE)`
- L616: `self.add(title)`
- L617: `subtitle = beat_subtitle("Die Norm-Heizlast nach DIN EN 12831-1", title)`
- L619: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L828: `_clear_stage(self, (title, subtitle, din))`
- L877: `title = scene_title(TITLE_DE)`
- L878: `self.add(title)`
- L879: `subtitle = beat_subtitle("Die Jahresdauerlinie", title)`
- L881: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1019: `_clear_stage(self, (title, subtitle, din))`
- L1072: `title = scene_title(TITLE_DE)`
- L1073: `self.add(title)`
- L1074: `subtitle = beat_subtitle("Auslegen auf die Spitze, nicht auf den Mittelwert", title)`
- L1076: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1227: `_clear_stage(self, (title, subtitle, din))`

### 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py
- L434: `scene_title`: `scene_title(TITLE_DE)`
- L435: `play_scene_title`: `play_scene_title(self, title)`
- L436: `beat_subtitle`: `beat_subtitle('Drei Bilanzgrenzen', title)`
- L636: `scene_title`: `scene_title(TITLE_DE)`
- L638: `beat_subtitle`: `beat_subtitle('Der nicht erneuerbare Primärenergiefaktor', title)`
- L953: `scene_title`: `scene_title(TITLE_DE)`
- L955: `beat_subtitle`: `beat_subtitle('Ein Gebäude, zwei Anlagen', title)`
- L1181: `scene_title`: `scene_title(TITLE_DE)`
- L1183: `beat_subtitle`: `beat_subtitle('Der Grenzwert: das Referenzgebäude', title)`
Title-related source lines:
- L22: `scene_title = _manim_module.scene_title`
- L23: `play_scene_title = _manim_module.play_scene_title`
- L25: `beat_subtitle = _manim_module.beat_subtitle`
- L434: `title = scene_title(TITLE_DE)`
- L435: `play_scene_title(self, title)`
- L436: `subtitle = beat_subtitle("Drei Bilanzgrenzen", title)`
- L438: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L636: `title = scene_title(TITLE_DE)`
- L637: `self.add(title)`
- L638: `subtitle = beat_subtitle("Der nicht erneuerbare Primärenergiefaktor", title)`
- L640: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L953: `title = scene_title(TITLE_DE)`
- L954: `self.add(title)`
- L955: `subtitle = beat_subtitle("Ein Gebäude, zwei Anlagen", title)`
- L957: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1181: `title = scene_title(TITLE_DE)`
- L1182: `self.add(title)`
- L1183: `subtitle = beat_subtitle("Der Grenzwert: das Referenzgebäude", title)`
- L1185: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🏷️certificate/🐍️.py
- L331: `scene_title`: `scene_title(TITLE_DE)`
- L332: `play_scene_title`: `play_scene_title(self, title)`
- L333: `beat_subtitle`: `beat_subtitle('Bedarf oder Verbrauch', title)`
- L530: `scene_title`: `scene_title(TITLE_DE)`
- L532: `beat_subtitle`: `beat_subtitle('Die Skala lesen', title)`
- L751: `scene_title`: `scene_title(TITLE_DE)`
- L753: `beat_subtitle`: `beat_subtitle('Was der Ausweis auslöst', title)`
Title-related source lines:
- L22: `scene_title = _manim_module.scene_title`
- L23: `play_scene_title = _manim_module.play_scene_title`
- L25: `beat_subtitle = _manim_module.beat_subtitle`
- L331: `title = scene_title(TITLE_DE)`
- L332: `play_scene_title(self, title)`
- L333: `subtitle = beat_subtitle("Bedarf oder Verbrauch", title)`
- L335: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L341: `return [FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din, caption)]`
- L479: `self.play(*[FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din)], run_time=0.6)`
- L530: `title = scene_title(TITLE_DE)`
- L531: `self.add(title)`
- L532: `subtitle = beat_subtitle("Die Skala lesen", title)`
- L534: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L554: `if m not in (title, subtitle, din, caption, *strip["bands"], *keep)]`
- L703: `self.play(*[FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din)], run_time=0.6)`
- L751: `title = scene_title(TITLE_DE)`
- L752: `self.add(title)`
- L753: `subtitle = beat_subtitle("Was der Ausweis auslöst", title)`
- L755: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L761: `return [FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din, caption, *keep)]`
- L913: `self.play(*[FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din)], run_time=0.6)`

### 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🔁️recap/🐍️.py
- L437: `scene_title`: `scene_title(TITLE_DE)`
- L438: `play_scene_title`: `play_scene_title(self, title)`
- L439: `beat_subtitle`: `beat_subtitle('Bilanzgrenze & Energieerhaltung', title)`
- L626: `scene_title`: `scene_title(TITLE_DE)`
- L628: `beat_subtitle`: `beat_subtitle('Die Verlustseite: Transmission & Lüftung', title)`
- L947: `scene_title`: `scene_title(TITLE_DE)`
- L949: `beat_subtitle`: `beat_subtitle('Die Gewinnseite & der Ausnutzungsgrad', title)`
- L1229: `scene_title`: `scene_title(TITLE_DE)`
- L1231: `beat_subtitle`: `beat_subtitle('Dieselbe Bilanz, zwei Jahreszeiten', title)`
Title-related source lines:
- L22: `scene_title = _manim_module.scene_title`
- L23: `play_scene_title = _manim_module.play_scene_title`
- L25: `beat_subtitle = _manim_module.beat_subtitle`
- L437: `title = scene_title(TITLE_DE)`
- L438: `play_scene_title(self, title)`
- L439: `subtitle = beat_subtitle("Bilanzgrenze & Energieerhaltung", title)`
- L441: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L626: `title = scene_title(TITLE_DE)`
- L627: `self.add(title)`
- L628: `subtitle = beat_subtitle("Die Verlustseite: Transmission & Lüftung", title)`
- L630: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L947: `title = scene_title(TITLE_DE)`
- L948: `self.add(title)`
- L949: `subtitle = beat_subtitle("Die Gewinnseite & der Ausnutzungsgrad", title)`
- L951: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1229: `title = scene_title(TITLE_DE)`
- L1230: `self.add(title)`
- L1231: `subtitle = beat_subtitle("Dieselbe Bilanz, zwei Jahreszeiten", title)`
- L1233: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py
- L306: `scene_title`: `scene_title(TITLE_DE)`
- L307: `play_scene_title`: `play_scene_title(self, title)`
- L308: `beat_subtitle`: `beat_subtitle('Warum Rechnung und Abrechnung auseinanderlaufen', title)`
- L721: `scene_title`: `scene_title(TITLE_DE)`
- L723: `beat_subtitle`: `beat_subtitle('Hydraulischer Abgleich', title)`
- L1078: `scene_title`: `scene_title(TITLE_DE)`
- L1080: `beat_subtitle`: `beat_subtitle('Die Serie im Rückblick', title)`
Title-related source lines:
- L22: `scene_title = _manim_module.scene_title`
- L23: `play_scene_title = _manim_module.play_scene_title`
- L25: `beat_subtitle = _manim_module.beat_subtitle`
- L306: `title = scene_title(TITLE_DE)`
- L307: `play_scene_title(self, title)`
- L308: `subtitle = beat_subtitle("Warum Rechnung und Abrechnung auseinanderlaufen", title)`
- L310: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L721: `title = scene_title(TITLE_DE)`
- L722: `self.add(title)`
- L723: `subtitle = beat_subtitle("Hydraulischer Abgleich", title)`
- L725: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L818: `gauge_title = math_label(r"\dot{V}/\dot{V}_{\mathrm{soll}}", at=np.array([-5.30, gauge_y - 0.09, 0.0]),`
- L820: `gauge_set = VGroup(gauge_title, *gauges)`
- L1011: `def _tile(cx, title, detail, colour):`
- L1015: `name = body_text(title, font_size=BODY_FONT_SIZE, color=colour).move_to(np.array([cx, -0.28, 0.0]))`
- L1078: `title = scene_title(TITLE_DE)`
- L1079: `self.add(title)`
- L1080: `subtitle = beat_subtitle("Die Serie im Rückblick", title)`
- L1082: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/☀️solar-gains/🐍️.py
- L384: `scene_title`: `scene_title(TITLE_DE)`
- L385: `play_scene_title`: `play_scene_title(self, title)`
- L386: `beat_subtitle`: `beat_subtitle('Von Wärmeverlust zu solarem Gewinn', title)`
- L513: `scene_title`: `scene_title(TITLE_DE)`
- L515: `beat_subtitle`: `beat_subtitle('Bestrahlungsstärke G und Fläche A', title)`
- L646: `scene_title`: `scene_title(TITLE_DE)`
- L648: `beat_subtitle`: `beat_subtitle('Gesamtenergiedurchlassgrad g', title)`
- L776: `scene_title`: `scene_title(TITLE_DE)`
- L778: `beat_subtitle`: `beat_subtitle('Saisonale Sonnenwinkel', title)`
- L895: `scene_title`: `scene_title(TITLE_DE)`
- L897: `beat_subtitle`: `beat_subtitle('Verschattungsfaktor', title)`
- L1119: `scene_title`: `scene_title(TITLE_DE)`
- L1121: `beat_subtitle`: `beat_subtitle('Hauptgleichung der solaren Gewinne', title)`
Title-related source lines:
- L33: `scene_title = _manim_module.scene_title`
- L34: `play_scene_title = _manim_module.play_scene_title`
- L36: `beat_subtitle = _manim_module.beat_subtitle`
- L384: `title = scene_title(TITLE_DE)`
- L385: `play_scene_title(self, title)`
- L386: `subtitle = beat_subtitle("Von Wärmeverlust zu solarem Gewinn", title)`
- L388: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L513: `title = scene_title(TITLE_DE)`
- L514: `self.add(title)`
- L515: `subtitle = beat_subtitle("Bestrahlungsstärke G und Fläche A", title)`
- L517: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L646: `title = scene_title(TITLE_DE)`
- L647: `self.add(title)`
- L648: `subtitle = beat_subtitle("Gesamtenergiedurchlassgrad g", title)`
- L650: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L776: `title = scene_title(TITLE_DE)`
- L777: `self.add(title)`
- L778: `subtitle = beat_subtitle("Saisonale Sonnenwinkel", title)`
- L780: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L895: `title = scene_title(TITLE_DE)`
- L896: `self.add(title)`
- L897: `subtitle = beat_subtitle("Verschattungsfaktor", title)`
- L899: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1119: `title = scene_title(TITLE_DE)`
- L1120: `self.add(title)`
- L1121: `subtitle = beat_subtitle("Hauptgleichung der solaren Gewinne", title)`
- L1123: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1154: `row.next_to(subtitle, DOWN, buff=0.95)`
- L1218: `VGroup(row, box).animate.next_to(subtitle, DOWN, buff=0.45).set_x(0),`
- L1219: `chain.animate.shift(UP * ((subtitle.get_bottom()[1] - 0.45) - box.get_top()[1])),`

### 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🌬️convection/🐍️.py
- L168: `scene_title`: `scene_title(TITLE_DE)`
- L169: `play_scene_title`: `play_scene_title(self, title)`
- L170: `beat_subtitle`: `beat_subtitle('Lüftungsverlust im Winter', title)`
Title-related source lines:
- L31: `scene_title = _manim_module.scene_title`
- L32: `play_scene_title = _manim_module.play_scene_title`
- L33: `beat_subtitle = _manim_module.beat_subtitle`
- L168: `title = scene_title(TITLE_DE)`
- L169: `play_scene_title(self, title)`
- L170: `subtitle = beat_subtitle("Lüftungsverlust im Winter", title)`
- L172: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/👥️internal-gains/🐍️.py
- L298: `scene_title`: `scene_title(TITLE_DE)`
- L299: `play_scene_title`: `play_scene_title(self, title)`
- L300: `beat_subtitle`: `beat_subtitle('Kalte Außenluft vs. interne Quellen', title)`
- L369: `scene_title`: `scene_title(TITLE_DE)`
- L371: `beat_subtitle`: `beat_subtitle('Personenabwärme', title)`
- L453: `scene_title`: `scene_title(TITLE_DE)`
- L455: `beat_subtitle`: `beat_subtitle('Geräteabwärme', title)`
- L571: `scene_title`: `scene_title(TITLE_DE)`
- L573: `beat_subtitle`: `beat_subtitle('Beleuchtungswärme', title)`
- L685: `scene_title`: `scene_title(TITLE_DE)`
- L687: `beat_subtitle`: `beat_subtitle('Summe und Wärmestromdichte', title)`
- L778: `beat_subtitle`: `beat_subtitle('Spezifische Wärmestromdichte (DIN V 18599-10)', title)`
Title-related source lines:
- L32: `scene_title = _manim_module.scene_title`
- L33: `play_scene_title = _manim_module.play_scene_title`
- L35: `beat_subtitle = _manim_module.beat_subtitle`
- L298: `title = scene_title(TITLE_DE)`
- L299: `play_scene_title(self, title)`
- L300: `subtitle = beat_subtitle("Kalte Außenluft vs. interne Quellen", title)`
- L301: `self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)`
- L369: `title = scene_title(TITLE_DE)`
- L370: `self.add(title)`
- L371: `subtitle = beat_subtitle("Personenabwärme", title)`
- L373: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L453: `title = scene_title(TITLE_DE)`
- L454: `self.add(title)`
- L455: `subtitle = beat_subtitle("Geräteabwärme", title)`
- L457: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L571: `title = scene_title(TITLE_DE)`
- L572: `self.add(title)`
- L573: `subtitle = beat_subtitle("Beleuchtungswärme", title)`
- L575: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L685: `title = scene_title(TITLE_DE)`
- L686: `self.add(title)`
- L687: `subtitle = beat_subtitle("Summe und Wärmestromdichte", title)`
- L688: `self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)`
- L778: `density_sub = beat_subtitle("Spezifische Wärmestromdichte (DIN V 18599-10)", title)`
- L780: `self.play(ReplacementTransform(subtitle, density_sub), run_time=0.7)`
- L781: `subtitle = density_sub`

### 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py
- L182: `scene_title`: `scene_title(text)`
- L334: `_heading`: `_heading('Übersicht der Wärmeverluste')`
- L434: `_heading`: `_heading('Vom Wärmestrom zur Jahresenergie')`
- L664: `_heading`: `_heading('Übersicht der Wärmegewinne')`
- L830: `_heading`: `_heading('Der Ausnutzungsgrad der Wärmegewinne')`
- L1080: `scene_title`: `scene_title('Die Hauptgleichung des Heizwärmebedarfs (DIN V 18599)')`
- L1330: `_heading`: `_heading('Anlagenverluste: vom Heizwärmebedarf zur Endenergie')`
Title-related source lines:
- L25: `scene_title = _manim_module.scene_title`
- L182: `return scene_title(text)`
- L334: `title = _heading("Übersicht der Wärmeverluste")`
- L355: `Write(title),`
- L393: `loss_title = math_label(r"\Phi_{\mathrm{Verlust}}", size=36).next_to(loss_desc, UP, buff=0.15)`
- L395: `loss_box = VGroup(loss_title, loss_desc, loss_slot).next_to(arrow, RIGHT, buff=0.25)`
- L400: `self.play(GrowArrow(arrow), FadeIn(VGroup(loss_title, loss_desc), shift=RIGHT * 0.2), *losses(2.0),`
- L434: `title = _heading("Vom Wärmestrom zur Jahresenergie")`
- L435: `self.play(Write(title), run_time=0.8)`
- L664: `title = _heading("Übersicht der Wärmegewinne")`
- L666: `subtitle = _label("Freie Wärmegewinne (DIN V 18599)", size=18, color=SUBTEXT_GREY)`
- L667: `subtitle.next_to(title, DOWN, buff=0.15)`
- L751: `self.play(Write(title), FadeIn(subtitle, shift=DOWN * 0.2), FadeIn(sol_room["group"]),`
- L830: `title = _heading("Der Ausnutzungsgrad der Wärmegewinne")`
- L831: `self.add(title)`
- L1043: `eta_title = VGroup(`
- L1047: `eta_title.move_to(DOWN * 0.8)`
- L1051: `eta_desc = VGroup(eta_line1, eta_line2).arrange(DOWN, buff=0.12).next_to(eta_title, DOWN, buff=0.3)`
- L1053: `self.play(Create(eta_box), FadeIn(eta_title), run_time=1.0)`
- L1080: `title = scene_title("Die Hauptgleichung des Heizwärmebedarfs (DIN V 18599)")`
- L1120: `self.play(Write(title), FadeIn(house["group"]), house["air"].animate.set_fill(opacity=0.14), run_time=1.2)`
- L1206: `self.play(FadeOut(VGroup(title, master_eq, shown, qh_read, expanded_eq, caption)), run_time=1.5)`
- L1330: `title = _heading("Anlagenverluste: vom Heizwärmebedarf zur Endenergie")`
- L1331: `self.play(Write(title), run_time=0.8)`

### 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🐍️.py
- L347: `scene_title`: `scene_title(TITLE_DE)`
- L348: `play_scene_title`: `play_scene_title(self, title)`
- L349: `beat_subtitle`: `beat_subtitle('Makroskopisch und mikroskopisch', title)`
- L412: `beat_subtitle`: `beat_subtitle('Mikroskopisches Gitter & Wärmebarriere', title)`
- L532: `scene_title`: `scene_title(TITLE_DE)`
- L534: `beat_subtitle`: `beat_subtitle('Wärmedurchlasswiderstand R', title)`
- L651: `scene_title`: `scene_title(TITLE_DE)`
- L653: `beat_subtitle`: `beat_subtitle('U-Wert und Temperaturgradient', title)`
- L840: `scene_title`: `scene_title(TITLE_DE)`
- L842: `beat_subtitle`: `beat_subtitle('Wie viel Watt gehen durch die Wand?', title)`
- L1002: `scene_title`: `scene_title(TITLE_DE)`
- L1004: `beat_subtitle`: `beat_subtitle('Die thermische Gebäudehülle', title)`
- L1200: `scene_title`: `scene_title(TITLE_DE)`
- L1202: `beat_subtitle`: `beat_subtitle('Wärmebrücken', title)`
Title-related source lines:
- L35: `scene_title = _manim_module.scene_title`
- L36: `play_scene_title = _manim_module.play_scene_title`
- L37: `beat_subtitle = _manim_module.beat_subtitle`
- L347: `title = scene_title(TITLE_DE)`
- L348: `play_scene_title(self, title)`
- L349: `subtitle = beat_subtitle("Makroskopisch und mikroskopisch", title)`
- L351: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L412: `micro_sub = beat_subtitle("Mikroskopisches Gitter & Wärmebarriere", title)`
- L416: `ReplacementTransform(subtitle, micro_sub),`
- L419: `subtitle = micro_sub`
- L532: `title = scene_title(TITLE_DE)`
- L533: `self.add(title)`
- L534: `subtitle = beat_subtitle("Wärmedurchlasswiderstand R", title)`
- L536: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L651: `title = scene_title(TITLE_DE)`
- L652: `self.add(title)`
- L653: `subtitle = beat_subtitle("U-Wert und Temperaturgradient", title)`
- L655: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L840: `title = scene_title(TITLE_DE)`
- L841: `self.add(title)`
- L842: `subtitle = beat_subtitle("Wie viel Watt gehen durch die Wand?", title)`
- L844: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1002: `title = scene_title(TITLE_DE)`
- L1003: `self.add(title)`
- L1004: `subtitle = beat_subtitle("Die thermische Gebäudehülle", title)`
- L1006: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1200: `title = scene_title(TITLE_DE)`
- L1201: `self.add(title)`
- L1202: `subtitle = beat_subtitle("Wärmebrücken", title)`
- L1204: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py
- L474: `scene_title`: `scene_title(TITLE_DE)`
- L475: `play_scene_title`: `play_scene_title(self, title)`
- L476: `beat_subtitle`: `beat_subtitle('Abschnitt 1.1 — Wie Wärme sich bewegt', title)`
- L626: `scene_title`: `scene_title(TITLE_DE)`
- L628: `beat_subtitle`: `beat_subtitle('Wärmeleitung — Energie ohne Materialtransport', title)`
- L738: `scene_title`: `scene_title(TITLE_DE)`
- L740: `beat_subtitle`: `beat_subtitle('Konvektion — bewegte Luft trägt die Wärme', title)`
- L850: `scene_title`: `scene_title(TITLE_DE)`
- L852: `beat_subtitle`: `beat_subtitle('Strahlung — braucht kein Medium', title)`
- L949: `scene_title`: `scene_title(TITLE_DE)`
- L951: `beat_subtitle`: `beat_subtitle('Zusammenfassung — drei Wege, ein Wärmestrom', title)`
- L1072: `scene_title`: `scene_title(TITLE_DE)`
- L1074: `beat_subtitle`: `beat_subtitle('Abschnitt 1.2 — Von drei Wegen zu einem Kennwert', title)`
- L1191: `scene_title`: `scene_title(TITLE_DE)`
- L1193: `beat_subtitle`: `beat_subtitle('Wärmedurchlasswiderstand R', title)`
- L1417: `scene_title`: `scene_title(TITLE_DE)`
- L1419: `beat_subtitle`: `beat_subtitle('U-Wert — wie leicht Wärme hindurchgeht', title)`
- L1572: `scene_title`: `scene_title(TITLE_DE)`
- L1574: `beat_subtitle`: `beat_subtitle('Wärmestrom durch ein Bauteil', title)`
Title-related source lines:
- L22: `scene_title = _manim_module.scene_title`
- L23: `play_scene_title = _manim_module.play_scene_title`
- L25: `beat_subtitle = _manim_module.beat_subtitle`
- L474: `title = scene_title(TITLE_DE)`
- L475: `play_scene_title(self, title)`
- L476: `subtitle = beat_subtitle("Abschnitt 1.1 — Wie Wärme sich bewegt", title)`
- L478: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L626: `title = scene_title(TITLE_DE)`
- L627: `self.add(title)`
- L628: `subtitle = beat_subtitle("Wärmeleitung — Energie ohne Materialtransport", title)`
- L630: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L738: `title = scene_title(TITLE_DE)`
- L739: `self.add(title)`
- L740: `subtitle = beat_subtitle("Konvektion — bewegte Luft trägt die Wärme", title)`
- L742: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L850: `title = scene_title(TITLE_DE)`
- L851: `self.add(title)`
- L852: `subtitle = beat_subtitle("Strahlung — braucht kein Medium", title)`
- L854: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L949: `title = scene_title(TITLE_DE)`
- L950: `self.add(title)`
- L951: `subtitle = beat_subtitle("Zusammenfassung — drei Wege, ein Wärmestrom", title)`
- L952: `self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)`
- L1072: `title = scene_title(TITLE_DE)`
- L1073: `self.add(title)`
- L1074: `subtitle = beat_subtitle("Abschnitt 1.2 — Von drei Wegen zu einem Kennwert", title)`
- L1076: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1191: `title = scene_title(TITLE_DE)`
- L1192: `self.add(title)`
- L1193: `subtitle = beat_subtitle("Wärmedurchlasswiderstand R", title)`
- L1195: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1417: `title = scene_title(TITLE_DE)`
- L1418: `self.add(title)`
- L1419: `subtitle = beat_subtitle("U-Wert — wie leicht Wärme hindurchgeht", title)`
- L1421: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`
- L1572: `title = scene_title(TITLE_DE)`
- L1573: `self.add(title)`
- L1574: `subtitle = beat_subtitle("Wärmestrom durch ein Bauteil", title)`
- L1576: `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`

### 🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py
- L678: `scene_title`: `scene_title(TITLE_DE)`
- L679: `play_scene_title`: `play_scene_title(self, title)`
- L680: `beat_subtitle`: `beat_subtitle('Leistung — wie schnell Energie fließt', title)`
- L1014: `scene_title`: `scene_title(TITLE_DE)`
- L1016: `beat_subtitle`: `beat_subtitle('Arbeit und Energie — die Kilowattstunde', title)`
- L1432: `scene_title`: `scene_title(TITLE_DE)`
- L1434: `beat_subtitle`: `beat_subtitle('Energie bleibt erhalten — jedes Watt wird Wärme', title)`
- L1803: `scene_title`: `scene_title(TITLE_DE)`
- L1805: `beat_subtitle`: `beat_subtitle('Wärmepumpe — Energie auf höheres Temperaturniveau übertragen', title)`
- L2186: `scene_title`: `scene_title(TITLE_DE)`
- L2188: `beat_subtitle`: `beat_subtitle('Strahlung — kurzwellig hinein, langwellig gefangen', title)`
- L2479: `scene_title`: `scene_title(TITLE_DE)`
- L2481: `beat_subtitle`: `beat_subtitle('Thermische Masse — speichern und zeitversetzt abgeben', title)`
- L2870: `scene_title`: `scene_title(TITLE_DE)`
- L2872: `beat_subtitle`: `beat_subtitle('Sensible und latente Wärme', title)`
- L3283: `scene_title`: `scene_title(TITLE_DE)`
- L3285: `beat_subtitle`: `beat_subtitle('Zusammenfassung — was wir gelernt haben', title)`
Title-related source lines:
- L25: `scene_title = _manim_module.scene_title`
- L26: `play_scene_title = _manim_module.play_scene_title`
- L27: `beat_subtitle = _manim_module.beat_subtitle`
- L678: `title = scene_title(TITLE_DE)`
- L679: `play_scene_title(self, title)`
- L680: `subtitle = beat_subtitle("Leistung — wie schnell Energie fließt", title)`
- L681: `self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)`
- L902: `chart_title = _m(r"\text{Heizleistung}\; P \;\text{über das Jahr (schematisch)}", size=LABEL_FONT_SIZE,`
- L915: `self.play(FadeIn(yax["group"]), FadeIn(months), FadeIn(p_ticks2), FadeIn(chart_title), run_time=0.8)`
- L1014: `title = scene_title(TITLE_DE)`
- L1015: `self.add(title)`
- L1016: `subtitle = beat_subtitle("Arbeit und Energie — die Kilowattstunde", title)`
- L1017: `self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)`
- L1432: `title = scene_title(TITLE_DE)`
- L1433: `self.add(title)`
- L1434: `subtitle = beat_subtitle("Energie bleibt erhalten — jedes Watt wird Wärme", title)`
- L1435: `self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)`
- L1704: `w_title = Text("Winter", font_size=BODY_FONT_SIZE, color=P_BLUE).next_to(w_frame, UP, buff=0.15)`
- L1711: `self.play(LaggedStart(*[GrowArrow(a) for a in loss_arrows], lag_ratio=0.2), FadeIn(w_frame), FadeIn(w_title),`
- L1734: `s_title = Text("Sommer", font_size=BODY_FONT_SIZE, color=P_YELLOW).move_to(`
- L1735: `np.array([sx, w_title.get_center()[1], 0.0]))`
- L1740: `self.play(FadeOut(loss_arrows), FadeIn(sun), FadeIn(s_title), run_time=0.7)`
- L1803: `title = scene_title(TITLE_DE)`
- L1804: `self.add(title)`
- L1805: `subtitle = beat_subtitle("Wärmepumpe — Energie auf höheres Temperaturniveau übertragen", title)`
- L1806: `self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)`
- L2023: `self.play(*[FadeOut(m) for m in self.mobjects if m not in (title, subtitle, caption)], run_time=0.7)`
- L2055: `self.play(*[FadeOut(m) for m in self.mobjects if m not in (title, subtitle, caption)], run_time=0.7)`
- L2186: `title = scene_title(TITLE_DE)`
- L2187: `self.add(title)`
- L2188: `subtitle = beat_subtitle("Strahlung — kurzwellig hinein, langwellig gefangen", title)`
- L2189: `self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)`
- L2479: `title = scene_title(TITLE_DE)`
- L2480: `self.add(title)`
- L2481: `subtitle = beat_subtitle("Thermische Masse — speichern und zeitversetzt abgeben", title)`
- L2482: `self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)`
- L2870: `title = scene_title(TITLE_DE)`
- L2871: `self.add(title)`
- L2872: `subtitle = beat_subtitle("Sensible und latente Wärme", title)`
- L2873: `self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)`
- L3283: `title = scene_title(TITLE_DE)`
- L3284: `self.add(title)`
- L3285: `subtitle = beat_subtitle("Zusammenfassung — was wir gelernt haben", title)`
- L3286: `self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)`
- L3336: `for who, book, subtitle, edition, link, topics, color in books:`
- L3339: `lines.append(Text(subtitle, font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).set_opacity(0.7))`

### 🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals/🐍️.py
- L117: `scene_title`: `_fonts.scene_title(self.TITLE[self.language()])`
Title-related source lines:
- L109: `content = [item for item in self.mobjects if item is not self._title and item is not self._caption]`
- L117: `self._title = _fonts.scene_title(self.TITLE[self.language()])`
- L118: `self.add(self._title)`
- L128: `Fixed slots: x title below the tick row, y title above the left edge, legend slot (``heat_map(legend=True)``) above the right edge.`
- L139: `y_title = _text(y_label, (0, point[1] + height / 2 + .4, 0), size=19, width=width * .5)`
- L140: `labels.add(y_title.shift(mn.RIGHT * (point[0] - width / 2 - .55 - y_title.get_left()[0])))`

### 🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py
- L133: `beat_subtitle`: `_fonts.beat_subtitle(self.tr(en, de), self._title)`
Title-related source lines:
- L90: `self._subtitle = None`
- L133: `new = _fonts.beat_subtitle(self.tr(en, de), self._title)`
- L134: `old, self._subtitle = self._subtitle, new`
- L425: `axes, y_title, y_labels, ticks = self._build_axes((0, Y_MAX, 500), [0, 500, 1000, 1500], self.tr("E (kWh/a)", "E (kWh/a)"))`
- L426: `x_title = self._l("Orientation θ of the glazed façade", "Ausrichtung θ der Glasfassade", (AXES_AT[0], -2.45, 0), size=17, width=AXES_SIZE[0] + .6)`
- L427: `m["axes"], m["axes_e"], m["y_labels_e"], m["y_title_e"], m["ticks"], m["x_title"] = axes, axes, y_labels, y_title, ticks, x_title`
- L435: `_mn.Create(axes), _mn.FadeIn(x_title), _mn.FadeIn(ticks), _mn.FadeIn(y_title), _mn.FadeIn(y_labels), _mn.FadeIn(marker), seconds=1.4)`
- L501: `def _compare_table(self, before, after, title, saved=False, extra=""):`
- L506: `head = [[(title, MUTED), ("", MUTED), ("", MUTED)]]`
- L613: `axes2, y_title2, y_labels2, _ = self._build_axes((-100, 500, 100), [0, 200, 400], self.tr("S (kWh/a)", "S (kWh/a)"))`
- L621: `self.step("negate", _mn.ReplacementTransform(m["curve"], curve2), _mn.FadeOut(m["axes"]), _mn.FadeOut(m["y_labels_e"]), _mn.FadeOut(m["y_title_e"]), _mn.FadeIn(axes2), _mn.FadeIn(y_labels2), _mn.FadeIn(y_title2),`
- L623: `m["axes"], m["y_labels_s"], m["y_title_s"], m["marker"], m["curve"] = axes2, y_labels2, y_title2, marker2, curve2`
- L651: `old = [m["axes"], m["y_labels_s"], m["y_title_s"], m["zero_tag"], m["worse"], m["saving"], m["curve"], m["ghost_bar"], m["ghost_tag"]]`
- L652: `self.step("constraint", *self._sub("Constraint: the quiet courtyard", "Nebenbedingung: der ruhige Hof"), *[_mn.FadeOut(item) for item in old], _mn.FadeIn(axes), _mn.FadeIn(m["y_labels_e"]), _mn.FadeIn(m["y_title_e"]), _mn.FadeIn(curve), _mn.FadeIn(m["fills"]),`
- L688: `keep = {id(self._title), id(self._caption), id(self._subtitle), id(m["footnote"])}`

### 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🐍️.py
- L83: `scene_title`: `_fonts.scene_title(self.TITLE[self.language()])`
Title-related source lines:
- L83: `title = _fonts.scene_title(self.TITLE[self.language()])`
- L84: `self.add(title)`

### 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py
- L176: `beat_subtitle`: `_fonts.beat_subtitle(self.tr('Villa Savoye · Ground floor', 'Villa Savoye · Erdgeschoss'), _fonts.scene_title(self.TITLE[self.language()]))`
- L176: `scene_title`: `_fonts.scene_title(self.TITLE[self.language()])`
Title-related source lines:
- L176: `return _fonts.beat_subtitle(self.tr("Villa Savoye · Ground floor", "Villa Savoye · Erdgeschoss"), _fonts.scene_title(self.TITLE[self.language()]))`
- L225: `head = 0.46 if title else 0.0`
- L233: `table.add(self._say(title, (left, top - 0.2), color=GOOD, edge="l", width=width - 0.3))`
- L412: `subtitle, norm = self._subtitle(), self._norm()`
- L415: `_engine.FadeIn(scale_bar), _engine.FadeIn(north), _engine.FadeIn(subtitle), _engine.FadeIn(norm), _engine.FadeIn(axon)), seconds=5.0)`

### 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧬️objects/🐍️.py
- L205: `beat_subtitle`: `_fonts.beat_subtitle(self.tr(en, de), _fonts.scene_title(self.TITLE[s.lang]))`
- L205: `scene_title`: `_fonts.scene_title(self.TITLE[s.lang])`
Title-related source lines:
- L205: `new = _fonts.beat_subtitle(self.tr(en, de), _fonts.scene_title(self.TITLE[s.lang]))`
- L283: `title = self.text(f"{m['building']['name'][lang]} · {m['site']['name'][lang]} · {years['design_start']}–{years['completed']}", (0, 0, 0), size=S_LABEL, color=INK)`
- L286: `lines = _engine.VGroup(title, *note).arrange(_engine.DOWN, buff=0.1)`
- L378: `s.plan_title = self.label("Ground-floor plan", "Grundriss Erdgeschoss", (6.8, -2.38, 0), size=S_SMALL, color=MUTED, anchor="r")`
- L390: `_engine.FadeIn(s.plan), _engine.FadeIn(s.plan_title), _engine.FadeIn(s.type_card), _engine.FadeIn(s.chip), _engine.FadeIn(s.relation),`
- L502: `old = _engine.VGroup(s.plan, s.lit, s.follow, s.ring, s.plan_title, s.pset, s.share, s.share_label, s.share_text)`
- L637: `title = self.label("type diameter: what-if", "Typdurchmesser: Was-wäre-wenn", (CHANGE.cx, 2.55, 0), size=S_SMALL, color=MUTED)`
- L641: `_engine.FadeIn(title), _engine.FadeIn(detail), run_time=1.0),`
- L643: `s.change_group = _engine.VGroup(s.total_group, detail, s.sum.product, s.sum.headline, s.sum.slab, s.sum.bars, s.sum_change.diameter, s.sum_change.area, s.sum_change.volume, guid, same, title)`
- L890: `title = self.label("section through the wall W-S", "Schnitt durch die Wand W-S", (3.0, 2.3, 0), size=S_LABEL, color=INK)`
- L892: `s.layer_group = _engine.VGroup(strip, names, total_dim, sides, title, cut)`
- L895: `_engine.AnimationGroup(_engine.FadeOut(gone), _engine.FadeIn(c.usage), _engine.FadeIn(c.material), _engine.FadeIn(c.layers), _engine.FadeIn(c.layer_assumed), _engine.FadeIn(title), run_time=1.3),`
- L986: `for title, x in zip(titles, (-4.4, 4.4)):`
- L989: `boxes.append(_engine.VGroup(frame, self.text(title, (x, 0.5, 0), size=S_LABEL, color=INK)))`
