# Presentation

A presentation consists of sequences.

# Sequence

A sequence of thoughts.

# Thought

A thought that develops over one or many slides.
A thought has participants.

# Participant

A participant appear on one or many slides.
A praticipant has different embodiments.

# Embodiment

An embodiment is a representation of a participant in a certain shape.
The embodiment has a default style which can be changed by dispositions.

# Figure

A figure embodiment of a participant.

# Video

A video embodiment of a participant.

# Text

A text embodiment of a participant.

# Pdf

A pdf embodiment of a participant.

# Disposition

A disposition is a concrete positioned, styled embodiment.
A figure disposition may declare a `split` of crop tiles (`SplitTile`), each with its own slide position and optional emphasis or style.

# Split

A split divides one figure into many independently placed tiles for reveal.js auto-animate (see `splitFigureGrid` in core). Set `concealed: true` on the split and add a whole-figure disposition on the same arrangement so tiles stay at full opacity (position-only morph) while the figure hides the grid. Group tiles with `split.columns`: tiles keep `tileMorphId` for grid morphs; `columnMorphTileGhosts` adds one hidden `columnMorphId` anchor per tile so every tile joins reveal’s group morph into `morphTargets`. Use a `columnGhostsOnly` merge arrangement (unified column slots) between focus and labels; merge→labels morphs all column ids into one label per column.

# Arrangement

An arrangement of dispositions.

# Transition

A transition from an arrangement into another arrangement.

# Template

A template is a parametric prefined sequence.

## Intro

Participants: title, description, goal, authors, affiliations

## Analogy

