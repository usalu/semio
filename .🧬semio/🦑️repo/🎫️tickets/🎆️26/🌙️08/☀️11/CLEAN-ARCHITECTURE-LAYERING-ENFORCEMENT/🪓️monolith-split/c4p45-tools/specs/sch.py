from specs.lib import *

CH = [
    ch('🧭', 'cursor-and-expressions', 'cursor_and_expressions', [(39, 333)],
       'Cursor and expressions of the schema parser: the source mode, the token cursor, the parse options, the entry points and the scalar and expression parsers.'),
    ch('🗺', 'shape-parsing', 'shape_parsing', [(335, 563)],
       'Shape parsing of the schema parser: the embed, field shape, quantity, number, dimension and shape parsers.'),
    ch('📜', 'dsl-value-and-wire', 'dsl_value_and_wire', [(565, 734)],
       'DSL value and wire parsing of the schema parser: the keyword probe, the DSL value parser and the wire parsers.'),
    ch('📒', 'records-and-tables', 'records_and_tables', [(736, 1000)],
       'Records and tables of the schema parser: the record body, call record and record field parsers and the table column and cell parsers.'),
    ch('🔎', 'canonicalize-and-language-service', 'canonicalize_and_language_service', [(1004, 1108)],
       'Canonical form and language service of the schema: the base64 decoder, the canonicalizer and the keyword completion service.'),
]

D1 = '''/// 📋️ Prints the braced AoS-list form `parse_table_list` reads back. Ordinary `[ ]` spacing
/// (a space just inside, per the general list rule — NOT the header's own tight-glued exception).
//#endregion 🔖️Table

fn base64_decode('''
D2 = '''//#endregion 🔖️Table

/// 📋️ Prints the braced AoS-list form `parse_table_list` reads back. Ordinary `[ ]` spacing
/// (a space just inside, per the general list rule — NOT the header's own tight-glued exception).
fn base64_decode('''


def pre(t):
    assert t.count(D1) == 1
    return t.replace(D1, D2)


SPEC = dict(
    orig='sch.rs',
    pre=pre,
    file=SCH + '/🦀️.rs',
    children=CH,
    external=mounted(SCH + '/🦀️.rs'),
)
