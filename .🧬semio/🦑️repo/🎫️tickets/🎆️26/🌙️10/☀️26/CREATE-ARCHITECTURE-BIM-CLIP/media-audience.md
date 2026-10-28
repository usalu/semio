# Encoded Video Audience and Language Audit

Independent PyAV decoding and Pillow/NumPy pixel analysis; no production module imports.

Complete four-variant topics checked: 6/6.
Pending topics: 0.
Failed checks: 0.

Glyph classifier: average RGB above 145; Speaker needs at least 80 bright pixels, Spectator at most max(40, 8% of Speaker), and the audience difference at least 60 pixels. Language caption mean RGB difference must exceed 1.0.

Frames use a shared narrated clause, independently located through each source trace. All selected instants lie in the middle 30–60% of the finished movie; measured envelope duration supplies the source offset.

| Video | Clause | English Speaker − Spectator Bright Pixels | German Speaker − Spectator Bright Pixels | EN/DE Caption Mean RGB Difference |
| --- | --- | ---: | ---: | ---: |
| bim_information | decision | 1612 | 1283 | 29.509 |
| bim_objects | methods | 2029 | 2237 | 16.825 |
| bim_representations | section | 1745 | 1529 | 30.273 |
| bim_models | schema | 2057 | 2003 | 27.217 |
| bim_exchange | purpose | 2053 | 2166 | 16.733 |
| bim_consistency | views | 2027 | 1784 | 25.173 |

## Expected Caption Text

- bim_information (decision):
  - EN: Information serves a question: is this wall suitable for the intended use?
  - DE: Information dient einer Frage: Eignet sich diese Wand für die geplante Nutzung?
- bim_objects (methods):
  - EN: A method describes behavior. An application calculates area and volume from attributes.
  - DE: Eine Methode beschreibt Verhalten. Die Anwendung berechnet Fläche und Volumen aus Attributen.
- bim_representations (section):
  - EN: The section shows height and thickness; the elevation shows length and height.
  - DE: Der Schnitt zeigt Höhe und Dicke; die Ansicht zeigt Länge und Höhe.
- bim_models (schema):
  - EN: Different models can share one schema: the same vocabulary and rules, with different content.
  - DE: Verschiedene Modelle können ein Schema teilen: gleiche Begriffe und Regeln, mit anderem Inhalt.
- bim_exchange (purpose):
  - EN: For our FEM use case, choose an analytical schema and idealize the wall as a middle surface.
  - DE: Für unser FEM-Beispiel wählen wir ein Analyseschema und idealisieren die Wand als Mittelfläche.
- bim_consistency (views):
  - EN: Derived views share the update. Area stays eighteen; volume becomes 7.2 cubic metres.
  - DE: Abgeleitete Darstellungen teilen die Änderung. Fläche bleibt achtzehn, Volumen wird 7,2 Kubikmeter.

Pixel differences verify visible audience/language distinctions; authored clauses and published certificates bind the expected language. This does not perform OCR.

Contact sheets and quantitative JSON live under ticket 🗑️generated/audience; the audit input script is preserved as audience-audit.py.

## Final Visual Readback

All 24 published files were sampled through six matched four-variant topics. All 12 Speaker caption regions contain visible glyphs (1283–2237 bright pixels). All 12 Spectator caption regions contain zero bright glyph pixels. The actual decoded sample positions range from 39.19% to 53.13% of their movies.

Manual readback of all six caption contact sheets confirmed the expected English and German text and blank Spectator caption bands. The final automated audit reports six complete topics, zero pending topics, and zero failures.

This audit verifies encoded visual captions and language; it makes no claim that an audio stream contains spoken narration. No prerecorded narration assets were supplied.
