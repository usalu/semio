from specs.lib import *

SPEC = dict(
    orig='ing.rs',
    file=ING + '/🦀️.rs',
    nest=False,
    keep_empty=('🧪️Tests',),
    external=[ING + '/🧪️tests/🔬️unit/🦀️.rs'],
    children=[
        ch('📄', 'command-page-behavior', 'command_page_behavior', [(30, 42), (58, 150)],
           'Command page behavior: the page block reservation, the fixed command page retirement and the fixed command page implementation with its value and serde codecs.', reexport=False),
        ch('🗃', 'page-set-and-reader-behavior', 'page_set_and_reader_behavior', [(176, 236), (247, 311), (319, 386)],
           'Command page set and reader behavior: the page set, the paged command and the paged command reader implementations.'),
        ch('📨', 'envelope-and-rejection-behavior', 'envelope_and_rejection_behavior', [(422, 487), (495, 528), (537, 594), (596, 619)],
           'Command envelope behavior: the envelope set, the rejected command build with its registry and the command batch implementations.'),
        ch('🧰', 'driver-registry-behavior', 'driver_registry_behavior', [(673, 847)],
           'Command driver registry behavior: the default and the driver registry implementation.'),
        ch('🚚', 'batch-driver-behavior', 'batch_driver_behavior', [(849, 1071)],
           'Command batch driver behavior: the batch driver implementation.'),
    ],
)
