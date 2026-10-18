from specs.lib import *
IP = PL + '/🧠️interpreter'
CH = [
    ch('🧱', 'core-types-and-module-model', 'core_types_and_module_model', [(21, 285)],
       'Core types and module model: the value types and values, the core error, host calls and step outcomes, the module declarations, limits and control bounds and the core module record.'),
    ch('🧩', 'component-artifact', 'component_artifact', [(291, 456)],
       'Component artifact: the component version, the artifact kinds and sections, the component limits and the wasm artifact with its section readers.'),
    ch('🔌', 'owned-semio-boundary', 'owned_semio_boundary', [(462, 745)],
       'Owned semio boundary: the checkpoint constants, the semio export names and the owned export enum, the owned artifact, instance and checkpoint layout.'),
    ch('🔎', 'binary-decoder-and-module-parser', 'binary_decoder_and_module_parser', [(751, 1184)],
       'Binary decoder and module parser: the byte decoder, the section order and the parsers of function types, limits, memories, tables, globals, constant expressions, code, elements and data.'),
    ch('🧭', 'instruction-shape', 'instruction_shape', [(1190, 1354)],
       'Instruction shape: the control analysis of function bodies, the immediate skippers and the block type decoder.'),
    ch('🏛', 'instance-state', 'instance_state', [(1364, 1636)],
       'Instance state: the core instance, its memory, table and global state, the machine, frames and control frames and the instance construction and call entry.'),
    ch('⚙', 'instruction-execution', 'instruction_execution', [(1642, 2011)],
       'Instruction execution: the instruction stepping loop, the branch targets, the stack helpers and the typed stack checks.'),
    ch('💾', 'memory-and-bulk-execution', 'memory_and_bulk_execution', [(2017, 2251)],
       'Memory and bulk execution: the memory and table instructions, the bulk memory operations and the little endian loads and stores.', reexport=False),
    ch('🧮', 'numeric-execution', 'numeric_execution', [(2257, 2581)],
       'Numeric execution: the numeric instruction dispatch and the unary, binary and comparison helpers.'),
    ch('🔢', 'numeric-semantics', 'numeric_semantics', [(2587, 2751)],
       'Numeric semantics: the canonical nans, the ties-even rounding, the wasm minimum and maximum and the truncating and saturating conversions.'),
    ch('📸', 'checkpoint-codec', 'checkpoint_codec', [(2761, 3135)],
       'Checkpoint codec: the borrowed checkpoint output retirement, the checkpoint writer and reader and the machine validation.', reexport=False),
]
_names = ['🦀️.rs'] + [c['folder'] + '/🦀️.rs' for c in CH]
_SRC_LINES = ['concat!('] + ['    include_str!("%s"),' % n for n in _names] + [')']
SRC = '\n'.join(_SRC_LINES)
SPEC = dict(source_concat=True, orig='ip.rs', file=IP + '/🦀️.rs', children=CH, wiring='block', external=mounted(IP + '/🦀️.rs', orig='ip.rs'),
            patch_parent=[('pub const INTERPRETER_SOURCE: &str = include_str!("🦀️.rs");', 'pub const INTERPRETER_SOURCE: &str = ' + SRC + ';')],
            expect_missing=['pub const INTERPRETER_SOURCE: &str = include_str!("🦀️.rs");'],
            expect_extra=['pub const INTERPRETER_SOURCE: &str = concat!('] + [l.strip() for l in _SRC_LINES[1:-1]] + [');'])
