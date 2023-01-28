import os
from json import loads

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))

with open(os.path.join(__location__, 'constants.json')) as constantsFile:
    constantsJson = constantsFile.read()

    CONSTANTS = loads(constantsJson)

    DEFAULT_GATEWAY_PORT = CONSTANTS['DEFAULT_GATEWAY_PORT']
    DEFAULT_ASSEMBLER_PORT = CONSTANTS['DEFAULT_ASSEMBLER_PORT']
    DEFAULT_MANAGER_PORT = CONSTANTS['DEFAULT_MANAGER_PORT']

    PLATFORMS = CONSTANTS['PLATFORMS']

    THREE = PLATFORMS['THREE']
    RHINO = PLATFORMS['RHINO']
    GRASSHOPPER = PLATFORMS['GRASSHOPPER']

    GENERAL_EXTENSIONS = CONSTANTS['GENERAL_EXTENSIONS']

    EXTENSIONS_BYPLATFORMURL = {platform['URL']:platform['EXTENSION'] for platform in PLATFORMS.values()}
    PLATFORMURL_BYEXTENSION = {platform['EXTENSION']:platform['URL'] for platform in PLATFORMS.values()}
    