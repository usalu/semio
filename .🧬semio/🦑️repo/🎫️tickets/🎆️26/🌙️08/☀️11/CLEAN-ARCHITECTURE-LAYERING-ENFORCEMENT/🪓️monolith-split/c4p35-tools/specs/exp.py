from specs.lib import *
import c4p35x as X

SPEC = dict(
    orig='exp.rs',
    file=EXP + '/🦀️.rs',
    build=X.build_exp,
    expect_missing=['mod analyze {', 'mod codegen {', 'mod parse {', '}', '}', '}'],
    children=[
        dict(folder='🔬' + V + 'analyze', mod='analyze'),
        dict(folder='🏗' + V + 'codegen', mod='codegen'),
        dict(folder='✂' + V + 'parse', mod='parse'),
    ],
)
