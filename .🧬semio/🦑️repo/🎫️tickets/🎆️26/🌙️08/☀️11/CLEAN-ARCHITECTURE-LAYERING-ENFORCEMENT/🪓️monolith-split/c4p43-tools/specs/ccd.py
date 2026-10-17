from specs.lib import *

CH = [
    ch('🧩', 'wire-combinators-and-writers', 'wire_combinators_and_writers', [(7, 242)],
       'Wire combinators and writers of the channel codec: the malformed error, the option and vector readers and writers, the command page writer and the media export handle codecs.'),
    ch('📤', 'command-encoder', 'command_encoder', [(244, 515)],
       'Command encoder of the channel codec: the tag dispatch that encodes every AppCommand variant.'),
    ch('📚', 'archive-and-pack-codecs', 'archive_and_pack_codecs', [(517, 686)],
       'Archive and pack codecs of the channel codec: the child pack and head, document archive and window config pack readers and writers.'),
    ch('📥', 'command-decoder', 'command_decoder', [(688, 821)],
       'Command decoder of the channel codec: the tag dispatch that decodes every AppCommand variant and the presence command sequence.'),
    ch('🎞', 'frame-codec', 'frame_codec', [(823, 1205)],
       'Frame codec of the channel codec: the AppFrame encoder and decoder and the local interaction query frame writer.'),
]

SPEC = dict(
    orig='ccd.rs',
    file=CCD + '/🦀️.rs',
    children=CH,
    nest=True,
    external=mounted(M + '/📡️spr/🧵️channel/🦀️.rs'),
)
