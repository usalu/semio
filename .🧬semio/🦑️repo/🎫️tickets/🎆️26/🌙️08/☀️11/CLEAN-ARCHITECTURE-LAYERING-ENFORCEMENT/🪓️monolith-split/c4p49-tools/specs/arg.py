from specs.lib import *

CH = [
    ch('📐', 'arg-format-and-schema', 'arg_format_and_schema', [(28, 244)],
       'Argument format and schema: the argument formats, the argument schema with its accessors and the interaction entity kind.'),
    ch('🎨', 'presentation-sources-and-reference-ids', 'presentation_sources_and_reference_ids', [(246, 349)],
       'Argument presentation, snap and option sources, number scale and the reference id type with its text form.'),
    ch('🎛', 'action-arg-control', 'action_arg_control', [(354, 581)],
       'Action argument control: the argument option and the control that renders and constrains one action argument.'),
    ch('🧾', 'action-arg-definition', 'action_arg_definition', [(583, 849)],
       'Action argument definition: the declared argument with its builders, validation and value coercion.'),
    ch('📜', 'format-application-and-json-schema', 'format_application_and_json_schema', [(851, 990)],
       'Format application and JSON schema of action arguments: the format check of one value and the JSON schema of an argument schema.'),
    ch('🔢', 'number-facets', 'number_facets', [(995, 1097)],
       'Number facets of action arguments: the declared bounds, step and unit, the unit symbol and the bound refusal.'),
    ch('🪪', 'app-role', 'app_role', [(1101, 1163)],
       'App role: the closed role of an app with its parsing and value, serde and DSL conversions.'),
]

SPEC = dict(
    orig='arg.rs',
    file=ARG + '/🦀️.rs',
    children=CH,
    wiring='each',
    external=[R + '/🛂️manifest/🧪️tests/🧪️number-facets/🦀️.rs'],
)
