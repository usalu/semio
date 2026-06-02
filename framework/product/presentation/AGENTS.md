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

A thought has one or many slides.

# Participant

A participant is either part of a presentation, a chapter, a sequence, a thought or a slide and is available only for the artifact and its children.
A participant appear on one or many slides.
A participant is represented with an embodiment inside a disposition.
Multiple participants can have the same embodiment.

# Embodiment

An embodiment is either part of a presentation, a chapter, a sequence, a thought or a slide and is available only for the artifact and its children.
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

# Demo

A demo embodiment of a pariticipant.
A demo is an iframe that when active has full control over keyboard etc.

# Disposition

A disposition is a concrete positioned, styled embodiment.
A disposition is ephemerally modifyable (hoverable, draggable, selectable, resizable and rotatetable).
A selection rectangle is started when the user clicks into empty stapce.
When the selection rectangle is going to the left, partial 
Because every disposition either disappears or morphs between slides, all ephemeral modification are lost after a slide changge.
You MUST NOT distort an embodiment. It is always a filled, scaled to the shorter-side, centered and covers an arbitrary target size.

# Morph

A morph is when one disposition changes to another disposition.
You MUST always start by first morphing the source embodiment from the the source position and source style into the target position and target style, and then afterwards switch embodiment from source to target.
You MUST NOT start by switching the embodiment, and then afterwards morphing the target embodiment from the the source position and source style into the target position and target style.
You MUST morph from the current disposition (including ephemeral modifications), although after the morph the ephemeral modifications will be gone.

# Arrangement

An arrangement of dispositions.

# Transition

A transition from an arrangement into another arrangement.

# Slide

A slide is an arragement with an optional transition to the next slide.

# Template

A template produces a set of matching artifacts (a presentation, a chapter, a sequence, a tought, a partcipant, embodiment, figure, video, text, pdf, disposition, split, morph, arrangement, transition, …)
Templates can use other templates.

## Intro

The intro template produces an sequence along with participants: title, description, goal, authors, affiliations.

## Tile

A tile template produces from a source figure a new figure embodiments.

## Split

A spit template produces a grid of tiles.