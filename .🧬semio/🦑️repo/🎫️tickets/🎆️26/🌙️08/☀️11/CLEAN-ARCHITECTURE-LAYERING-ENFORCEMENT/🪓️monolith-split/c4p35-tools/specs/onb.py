from specs.lib import *

SPEC = dict(
    orig='onb.rs',
    file=ONB + '/🦀️.rs',
    nest=False,
    keep_empty=('🧪️Tests',),
    external=[ONB + '/🧪️tests/🦀️.rs'],
    children=[
        ch('🎬', 'introduction-definition', 'introduction_definition', [(19, 29), (34, 145)],
           'Introduction definition: the start action, the introduction and its step definitions with their step implementation.'),
        ch('🖱', 'introduction-interaction', 'introduction_interaction', [(147, 497)],
           'Introduction interaction: the logo, the placement, the interaction kinds, the points, the pointer buttons and key modifiers, the gestures, the cursors and the demonstration.'),
        ch('📚', 'tutorial-definition', 'tutorial_definition', [(502, 708)],
           'Tutorial definition: the tutorial, its chapters, base and tracks, the asset source, the narration cues, the captions and the overlay rectangle.'),
        ch('🎞', 'tutorial-events-and-ui', 'tutorial_events_and_ui', [(710, 1024)],
           'Tutorial events and interface timeline: the video cues, the events, the interface keyframes, samples, snapshots and changes, the document events, the camera keyframes and states, the easing and the gesture cues.'),
        ch('🎥', 'tutorial-engine', 'tutorial_engine', [(1026, 1301)],
           'Tutorial engine: the start and record actions, the validation, the camera interpolation, the interface composition and the tutorial slice.'),
    ],
)
