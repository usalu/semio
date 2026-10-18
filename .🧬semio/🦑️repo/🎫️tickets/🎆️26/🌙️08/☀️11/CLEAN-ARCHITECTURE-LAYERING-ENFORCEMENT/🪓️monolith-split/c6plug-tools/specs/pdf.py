from specs.lib import *
PM = '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📐️model'
CH = [
    ch('🔗', 'object-model', 'object_model', [(11, 331)],
       'Object model of the PDF document model: the indirect reference, dictionary entries, stream filters and predictors, decimals, objects, indirect objects and the geometry aliases.'),
    ch('🖋', 'content-operators', 'content_operators', [(335, 507)],
       'Content operators of the PDF document model: the text strings and arrays, property lists, inline images, line styles and the operator enum.'),
    ch('🌈', 'colour-and-graphics-state', 'colour_and_graphics_state', [(511, 734)],
       'Colour and graphics state of the PDF document model: the colour spaces, functions, shadings, patterns, soft masks, extended graphics states and the named colour space and property resources.'),
    ch('🔤', 'font-model', 'font_model', [(738, 1038)],
       'Fonts of the PDF document model: the encodings, descriptors, programs, ToUnicode maps, CMaps, CID fonts and the font kinds.'),
    ch('🖼', 'xobjects', 'xobjects', [(1042, 1151)],
       'XObjects of the PDF document model: the image bodies and masks, images, transparency groups and form XObjects.'),
    ch('🎯', 'navigation', 'navigation', [(1155, 1326)],
       'Navigation of the PDF document model: the destinations, file specifications, actions, outline items, named destinations and page labels.'),
    ch('📌', 'annotations-and-forms', 'annotations_and_forms', [(1330, 1553)],
       'Annotations and forms of the PDF document model: the appearances, border styles, markup annotations, annotation kinds, form fields and the interactive form.'),
    ch('📄', 'document-metadata', 'document_metadata', [(1557, 1828)],
       'Document metadata of the PDF document model: the optional content, dates, information dictionary, embedded files, output intents, viewer preferences, open action, encryption and mark info.'),
    ch('📃', 'page-model', 'page_model', [(1832, 1950)],
       'Page of the PDF document model: the page record with its defaults and accessors.'),
]
SPEC = dict(orig='pdf.rs', file=PM + '/🦀️.rs', children=CH, external=mounted(PM + '/🦀️.rs', orig='pdf.rs'), nest=True)
