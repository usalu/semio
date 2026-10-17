import os, sys, json
T = os.path.dirname(os.path.abspath(__file__))
S = '✏️s/🔌️plugins/🗄️stdio'
V = '️'
A = S + '/🗿️artifacts'


def ch(emoji, kebab, mod, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, ranges=ranges, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)


def un(name, emoji, kebab, doc, children, **kw):
    return dict(name=name, folder=emoji + V + kebab, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), children=children, **kw)


P21 = S + '/📇️registry/🧬️contract/📐️part21'
DOC = S + '/🔮️oracles/📃️document'
DIFF = A + '/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🔺️diff'
ZIP = A + '/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io'
MP4 = A + '/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io'
BCF = A + '/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🔮️oracles'
STEP = A + '/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📐️geometry'
XLSX = A + '/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles'

TABLE = {
 'part21': dict(orig='part21.rs.txt', file=P21 + '/🦀️.rs', external=[P21 + '/🧪️tests/🔬️unit/🦀️.rs', P21 + '/🚦️controlled/🦀️.rs'], children=[
    ch('🔢', 'values', 'values', [(22, 193)], 'Exact values of the Part-21 codec: the decimal real, the value tree and the instance record.'),
    ch('📄', 'document-model', 'document_model', [(197, 353)], 'Document model of the Part-21 codec: the header with its attribute tables, the document, the builder and the error type.'),
    ch('🔤', 'lexer-and-parse', 'lexer_and_parse', [(357, 807)], 'Lexer and parser of the Part-21 codec: the tokenizer over the exchange structure and the document parse entry.'),
    ch('✍', 'writer', 'writer', [(811, 1029)], 'Writer of the Part-21 codec: the canonical text writer with its preamble hook and the string escaping.'),
    ch('🧾', 'value-and-canonical-codecs', 'value_and_canonical_codecs', [(1033, 1282)], 'Value and canonical-tree codecs of the Part-21 model: the value projections of the header, document, value and instance and the canonical JSON trees.'),
 ]),
 'document': dict(orig='document.rs.txt', file=DOC + '/🦀️.rs', external=[], expect_missing=['pub mod ooxml {', 'pub mod pdf_conformance {', '}', '}'], children=[
    ch('📕', 'pdf-spec-and-oracles', 'pdf_spec_and_oracles', [(13, 239)], 'PDF oracles of the document test oracle: the PDF spec, the creation and editing oracles and the independent projection.'),
 ], unwrap=[
    un('ooxml', '🗂', 'ooxml', 'The OOXML conformance-class engine of the document test oracle: the OPC container, the XML edits, the content types, the package facts, the wire form and the conformance mutations.', [
        ch('📦', 'container-and-xml-edits', 'container_and_xml_edits', [(272, 562)], 'Container and XML edits of the OOXML engine: the independent zip read and write and the root element edits.'),
        ch('🏷', 'content-types-and-package-facts', 'content_types_and_package_facts', [(567, 805)], 'Content types and package facts of the OOXML engine: the content-type map, the main part, the relationship types and the declared namespaces.'),
        ch('🔌', 'package-wire-and-reading', 'package_wire_and_reading', [(807, 1124)], 'Package wire and reading of the OOXML engine: the logical XML parts, their arena and wire form and the independent reading.'),
        ch('🏅', 'conformance-mutations-and-projection', 'conformance_mutations_and_projection', [(1128, 1408)], 'Conformance mutations and projection of the OOXML engine: the profile, the namespace and relationship rewrites, the arrange and inverse specs and the projection.'),
    ]),
    un('pdf_conformance', '🧷', 'pdf-conformance', 'The PDF conformance-class engine of the document test oracle: the profile, the object-graph scans and builders, the forward mutations and the arrange, inverse and projection halves.', [
        ch('🔎', 'profile-container-and-scans', 'profile_container_and_scans', [(1443, 1795)], 'Profile, container and scans of the PDF engine: the conformance profile, the lopdf loading, the object-graph scans and the object builders.'),
        ch('⚙', 'forward-mutation', 'forward_mutation', [(1799, 2186)], 'Forward mutation of the PDF engine: the conformance mutation entry and its in-place application.'),
        ch('🔁', 'arrange-inverse-and-projection', 'arrange_inverse_and_projection', [(2191, 2419)], 'Arrange, inverse and projection of the PDF engine: the arrange spec, the inverse spec and the semantic projection.'),
    ]),
 ]),
 'header_diff': dict(orig='header_diff.rs.txt', file=DIFF + '/🦀️.rs', external=[DIFF + '/🧪️tests/🔬️unit/🦀️.rs'], children=[
    ch('🧮', 'indexed-triple-algebra', 'indexed_triple_algebra', [(18, 182)], 'Indexed triple algebra of the LAS diff: the target validation, the label simulation and the absorb and inverse of removed, modified and added rows.'),
    ch('🧱', 'vlr-diff', 'vlr_diff', [(186, 328)], 'Variable length record diff of the LAS diff: the sparse record diff, its rows and the absorb and inverse of the record collection.'),
    ch('📍', 'point-diff', 'point_diff', [(332, 588)], 'Point diff of the LAS diff: the sparse point diff, its rows and the absorb and inverse of the point collection.'),
    ch('🔄', 'diff-algebra-and-builders', 'diff_algebra_and_builders', [(681, 1022)], 'Diff algebra and builders of the LAS diff: the header diff application, the mutation diff and diff algebra implementations and the typed diff constructors.'),
 ]),
 'zip_io': dict(orig='zip_io.rs.txt', file=ZIP + '/🦀️.rs', external=[ZIP + '/🧪️tests/🔬️codec/🦀️.rs'], children=[
    ch('🧮', 'byte-primitives-and-text', 'byte_primitives_and_text', [(60, 372)], 'Byte primitives and text of the zip codec: the compression method, the CRC32, the error type, the little-endian readers, the CP437 and UTF-8 member text decoding and the ZIP64 and Unicode extra fields.'),
    ch('🔍', 'inspect-decode-and-sniff', 'inspect_decode_and_sniff', [(376, 699), (1091, 1116)], 'Inspect, decode and sniff of the zip codec: the central entry header inspection, the end of central directory lookup, the archive decode and the byte sniff.'),
    ch('📮', 'encode', 'encode', [(703, 1087)], 'Encode of the zip codec: the member header encoding, the snapshot validation, the ordered archive encoding and the document archive helpers.'),
 ]),
 'mp4_io': dict(orig='mp4_io.rs.txt', file=MP4 + '/🦀️.rs', external=[MP4 + '/🧪️tests/🔬️codec/🦀️.rs'], children=[
    ch('📊', 'sample-tables', 'sample_tables', [(87, 299)], 'Sample tables of the MP4 codec: the sniff and the stts, ctts, stsc, stsz, stco, co64 and stss sample table parsing with the run-length helpers.'),
    ch('🎞', 'box-parsing-and-decode', 'box_parsing_and_decode', [(303, 667)], 'Box parsing and decode of the MP4 codec: the track, media and movie header boxes, the codec configuration boxes and the movie decode.'),
    ch('🧱', 'box-builders', 'box_builders', [(671, 993)], 'Box builders of the MP4 codec: the sample entry, codec configuration, sample table, header and movie box builders.'),
    ch('🗂', 'encode-plan', 'encode_plan', [(995, 1166)], 'Encode plan of the MP4 codec: the whole-file encode, the track plan, the measure and emit phases and the box size helpers.'),
    ch('🧭', 'encode-cursor', 'encode_cursor', [(1168, 1702)], 'Encode cursor of the MP4 codec: the bounded incremental mux with its checkpointed state and the HEVC and metadata header helpers.'),
 ]),
 'bcf_oracles': dict(orig='bcf_oracles.rs.txt', file=BCF + '/🦀️.rs', external=[], expect_missing=['mod oracles {', '}'], children=[], unwrap=[
    un('oracles', '🧰', 'engine', 'Independent BCF 2.1 oracle engine: the model, the XML tree, the zip archive, the markup and visualization XML, the JSON codec and the forward and inverse mutations.', [
        ch('📐', 'model-and-xml', 'model_and_xml', [(49, 331)], 'Model and XML of the BCF oracle: the independent BCF document model, the XML tree, its parser and writer and the archive entries.'),
        ch('💬', 'markup-and-viewpoint-xml', 'markup_and_viewpoint_xml', [(335, 579)], 'Markup and viewpoint XML of the BCF oracle: the markup.bcf and visualization info parse and write.'),
        ch('🧬', 'document-codec-and-json', 'document_codec_and_json', [(583, 879)], 'Document codec and JSON of the BCF oracle: the archive decode and encode and the JSON value mapping of the model.'),
        ch('🦠', 'forward-and-inverse', 'forward_and_inverse', [(883, 1159)], 'Forward and inverse of the BCF oracle: the mutation kinds applied to the model and their computed inverse specs.'),
        ch('🧭', 'routing-and-projection', 'routing_and_projection', [(1163, 1247)], 'Routing and projection of the BCF oracle: the mutation entry points, the round trip and the semantic projection.'),
    ]),
 ]),
 'step_geometry': dict(orig='step_geometry.rs.txt', file=STEP + '/🦀️.rs', external=[STEP + '/🧪️tests/🔬️unit/🦀️.rs'], children=[
    ch('✍', 'writer', 'writer', [(82, 450)], 'Writer of the STEP geometry codec: the product ids, the write context that emits the entity graph and the numeric helpers.'),
    ch('📖', 'entity-parsing', 'entity_parsing', [(456, 783)], 'Entity parsing of the STEP geometry codec: the entity table, the slot, reference and number parsers and the B-spline attribute readers.'),
    ch('🏗', 'solid-builder', 'solid_builder', [(789, 1117)], 'Builder of the STEP geometry codec: the step builder that resolves the parsed entities into a solid.'),
 ]),
 'xlsx_oracles': dict(orig='xlsx_oracles.rs.txt', file=XLSX + '/🦀️.rs', external=[XLSX + '/🧪️tests/🔬️unit/🦀️.rs'], children=[
    ch('🔢', 'grid-and-cell-addressing', 'grid_and_cell_addressing', [(29, 307)], 'Grid and cell addressing of the XLSX oracle: the workbook grid values, the spec readers and the A1 cell address arithmetic.'),
    ch('🧮', 'mutation-dispatch', 'mutation_dispatch', [(311, 476)], 'Mutation dispatch of the XLSX oracle: the forward mutation, the arrange, the inverse and the round trip.'),
    ch('🧵', 'shared-string-pool', 'shared_string_pool', [(481, 696)], 'Shared strings of the XLSX oracle: the shared string pool codec, its projection and its inverse spec.'),
    ch('🔌', 'plumbing-and-projection', 'plumbing_and_projection', [(701, 1048)], 'Plumbing and projection of the XLSX oracle: the package plumbing and the workbook and plumbing projections.'),
 ]),
}

if __name__ == '__main__':
    for k, v in TABLE.items():
        sp = dict(orig=v['orig'], file=v['file'], nest=False, children=v['children'], external=v['external'])
        if v.get('unwrap'):
            sp['unwrap'] = v['unwrap']
        for key in ('expect_missing', 'patch', 'patch_parent', 'expect_extra'):
            if v.get(key):
                sp[key] = v[key]
        with open(os.path.join(T, 'specs', k + '.py'), 'w', encoding='utf-8') as fh:
            fh.write('SPEC = %s\n' % json.dumps(sp, ensure_ascii=False, indent=1).replace('true', 'True').replace('false', 'False'))
        print('wrote', k)
