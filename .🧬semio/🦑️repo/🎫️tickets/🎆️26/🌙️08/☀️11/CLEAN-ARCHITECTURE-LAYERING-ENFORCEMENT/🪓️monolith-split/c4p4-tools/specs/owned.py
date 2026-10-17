F = '✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🦀️.rs'

IMPL = 'impl DrawingMutationCandidateAuthority {'
CUTS = [451, 970]


def pre(text):
    lines = text.split('\n')
    for cut in sorted(CUTS, reverse=True):
        assert lines[cut - 1] == '', (cut, lines[cut - 1])
        assert lines[cut].lstrip().startswith('pub fn '), (cut, lines[cut])
        lines[cut - 1:cut] = ['}', '', IMPL]
    junk = [i for i, line in enumerate(lines[:80]) if line == '>;']
    assert junk == [68, 69, 70], junk
    for i in reversed(junk):
        del lines[i]
    return '\n'.join(lines)


PRE_EXTRA = ['}', '}', IMPL, IMPL]

SPEC = dict(
    orig='owned.rs.txt',
    file=F,
    pre=pre,
    pre_extra=PRE_EXTRA,
    children=[
        dict(folder='🧩️mutation-candidate', mod='mutation_candidate',
             ranges=[(71, 448), (971, 1213)],
             doc='//! 🧩️ Drawing mutation candidate: the candidate phase machine state, the overlay patch, the source access, the clone source projections and the candidate authority with its construction, rebuild, retirement and close steps.'),
        dict(folder='🪜️candidate-step', mod='candidate_step',
             ranges=[(450, 969)],
             doc='//! 🪜️ Drawing candidate step: the `step` of the mutation candidate authority that applies one drawing mutation to the candidate snapshot in bounded turns.'),
        dict(folder='🚀️store-initialization', mod='store_initialization',
             ranges=[(1215, 1591), (2074, 2151)],
             doc='//! 🚀️ Drawing store initialization: the initialization phase, the initialization authority with its sealing and close steps, the plugin authority implementation and the initialization job.'),
        dict(folder='🔄️initialization-turn', mod='initialization_turn',
             ranges=[(1593, 2072)],
             doc='//! 🔄️ Drawing initialization turn: the turn outcome and the `turn` of the store initialization authority that drives its phase machine one bounded turn at a time.'),
    ],
    external='auto',
)
