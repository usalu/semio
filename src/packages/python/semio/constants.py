import os
from json import loads

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))

with open(os.path.join(__location__, 'constants.json')) as constantsFile:
    constantsJson = constantsFile.read()

    CONSTANTS = loads(constantsJson)

    DEFAULT_GATEWAY_PORT = CONSTANTS['DEFAULT_GATEWAY_PORT']
    DEFAULT_MANAGER_PORT = CONSTANTS['DEFAULT_MANAGER_PORT']

    PLATFORMS = CONSTANTS['PLATFORMS']

    THREE = PLATFORMS['three']
    RHINO = PLATFORMS['rhino']
    GRASSHOPPER = PLATFORMS['gh']
    