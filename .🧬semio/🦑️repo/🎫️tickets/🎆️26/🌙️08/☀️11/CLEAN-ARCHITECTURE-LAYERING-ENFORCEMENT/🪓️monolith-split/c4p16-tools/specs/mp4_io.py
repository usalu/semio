SPEC = {'orig': 'mp4_io.rs.txt',
 'file': '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs',
 'nest': False,
 'children': [{'folder': '🎞️box-parsing-and-decode',
               'mod': 'box_parsing_and_decode',
               'ranges': [[87, 299], [303, 667]],
               'doc': '//! 🎞️ Box parsing and decode of the MP4 codec: the sniff, the sample table parsing (stts, ctts, stsc, stsz, stco, co64, stss) with the run-length helpers, the track, media '
                      'and movie header boxes, the codec configuration boxes and the movie decode. See [`super`].'},
              {'folder': '🧱️box-builders',
               'mod': 'box_builders',
               'ranges': [[671, 993]],
               'doc': '//! 🧱️ Box builders of the MP4 codec: the sample entry, codec configuration, sample table, header and movie box builders. See [`super`].'},
              {'folder': '🧭️encode',
               'mod': 'encode',
               'ranges': [[995, 1166], [1168, 1702]],
               'doc': '//! 🧭️ Encode of the MP4 codec: the whole-file encode, the track plan, the measure and emit phases, the box size helpers, and the bounded incremental mux with its checkpointed '
                      'state and the HEVC and metadata header helpers. See [`super`].'}],
 'external': ['✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️codec/🦀️.rs']}
