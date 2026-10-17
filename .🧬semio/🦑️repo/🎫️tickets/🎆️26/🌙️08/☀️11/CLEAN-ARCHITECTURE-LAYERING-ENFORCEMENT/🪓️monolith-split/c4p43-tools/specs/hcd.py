from specs.lib import *


def pre(text):
    assert text.endswith('//#region 🧪️Tests\n')
    return text + '//#endregion 🧪️Tests\n'


CH = [
    ch('📝', 'text-grammar', 'text_grammar', [(20, 258)],
       'Text grammar of the history codec: the field identifiers and record specs of the doc, edit and transition lines and the ops text parser and printer.'),
    ch('🧱', 'field-and-message-codecs', 'field_and_message_codecs', [(262, 388)],
       'Field primitives and message codecs of the history codec: the string and id field readers and writers, the history message codec and the doc header codec.'),
    ch('✏', 'op-and-edit-payloads', 'op_and_edit_payloads', [(392, 681)],
       'Operation and edit payloads of the history codec: the operation payload and metadata readers and writers and the edit record codec.'),
    ch('🔀', 'transition-and-conflict-payloads', 'transition_and_conflict_payloads', [(685, 968)],
       'Transition, composition, conflict and viewer payloads of the history codec, with the dictionary record applier.'),
    ch('🗂', 'offset-index', 'offset_index', [(971, 1117)],
       'Offset index of the history codec: the section identifiers, the pair and offset section writers and the index builder and reader.'),
]

SPEC = dict(
    orig='hcd.rs',
    file=HCD + '/🦀️.rs',
    children=CH,
    external=mounted(M + '/📡️spr/📜️history/🦀️.rs'),
    pre=pre,
    pre_extra=['//#endregion 🧪️Tests'],
    keep_empty=('🧪️Tests',),
)
