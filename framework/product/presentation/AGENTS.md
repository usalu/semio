# Presentation

A presentation consists of chapters.
The slides are presented by a grid.
When `escape` is pressed, the overview is shown.
The presentation is website under site `/{columnIndex}/{rowIndex}?chapter={chapterName}&sequence={sequenceName}&thought={thoughtName}&slide={slideName}`. The variables are only for humans and not used by the system.

# Column

A vertical list of slides.

# Row

The slide in a column.

# Chapter

A chapter of sequences.

# Sequence

A sequence of thoughts.
A sequence is presented as a column.

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

# Split

A split divides one figure into many independently placed tiles.

# Morph

A morph is when one disposition changes to another disposition.
You MUST always start by first morphing the source embodiment from the the source position and source style into the target position and target style, and then afterwards switch embodiment from source to target.
You MUST NOT start by switching the embodiment, and then afterwards morphing the target embodiment from the the source position and source style into the target position and target style.

# Arrangement

An arrangement of dispositions.

# Transition

A transition from an arrangement into another arrangement.

# Slide

A slide is an arragement with an optional transition to the next slide.

# Template

A template produces anything parametrically (a presentation, a chapter, a sequence, a tought, a partcipant, embodiment, figure, video, text, pdf, disposition, split, morph, arrangement, transition, …)

## Intro

Produces an intro sequence.

Participants: title, description, goal, authors, affiliations
