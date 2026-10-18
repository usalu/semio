from specs.lib import *
F = STORE + '/📬️command/🦀️.rs'
CH = [
    ch('📜', 'ops-header-lines', 'ops_header_lines', [(18, 247)],
       'Ops header lines: the ops author, pin and superseded input, their conversions and the ops header line with its text and binary codecs.'),
    ch('🏷', 'command-header-supersede-inputs-and-policy-tokens', 'command_header_supersede_inputs_and_policy_tokens', [(249, 665)],
       'Command header, supersede inputs and policy tokens: the command header line, the supersede input lines with their print, parse, write and read, and the undo policy, history lane, merge policy and conflict resolution tokens and ordinals.'),
    ch('📝', 'command-print-parse-and-codec', 'command_print_parse_and_codec', [(667, 980)],
       'Command print, parse and codec: the indented ops, the command print and parse, the transaction text and the command write and read over strings and operations.'),
    ch('🧯', 'command-decode-error-and-retirement', 'command_decode_error_and_retirement', [(982, 1078), (1085, 1104)],
       'Command decode error and retirement: the command decode error with its conversions, the operation and command retirement, the operation envelope serde and the scratch retirements.'),
]
SPEC = dict(orig='cmd.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='cmd.rs'))
