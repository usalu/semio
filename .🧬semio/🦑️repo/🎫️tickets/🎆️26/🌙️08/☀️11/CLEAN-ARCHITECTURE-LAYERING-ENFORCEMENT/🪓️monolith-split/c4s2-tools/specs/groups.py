from specs.lib import *
PETS = '🧰️framework/🛍️products/🐾️pets'

RUNS = {
    'pet': ('semio_framework_pets_traversal', PETS + '/🔨️modules/🧭️traversal/🦀️.rs', ['lib', 'test']),
    'tri': ('semio_s_dev_trinity_jack', TRI.rsplit('/🧪️tests', 1)[0] + '/🦀️.rs', ['lib', 'test']),
    'srvt': ('semio_framework_server_tests', SRV + '/🧪️tests/🥣️companion/🦀️.rs', ['test']),
}
GROUPS = {
    'pet': dict(copy=[PETS + '/🔨️modules/🧭️traversal', PETS + '/🔨️modules/🧗️climbing', PETS + '/🔨️modules/🪢️swing'], specs=['pet'], runs=['pet'], frag=False),
    'tri': dict(copy=[TRI.rsplit('/🧪️tests', 1)[0]], specs=['tri'], runs=['tri'], frag=False),
    'gwu': dict(copy=[SRV + '/🔨️modules', SRV + '/🧪️tests/🥣️companion', SRV + '/🧪️tests/🧩️instance'], specs=['gwu'], runs=['srvt'], frag=False),
}
