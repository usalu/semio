import os
from json import loads
from types import SimpleNamespace
from model import Platform

__location__ = os.path.realpath(
    os.path.join(os.getcwd(), os.path.dirname(__file__)))

with open(os.path.join(__location__, 'constants.json')) as constantsFile:
    constantsJson = constantsFile.read()

    CONSTANTS = loads(constantsJson,object_hook=lambda d: SimpleNamespace(**d))

    SERVICES = CONSTANTS.SERVICES

    SERVICES_BYNAME = {SERVICES.__dict__[SERVICE].NAME:SERVICES.__dict__[SERVICE] for SERVICE in SERVICES.__dict__}

    GATEWAY = SERVICES.GATEWAY
    GATEWAY_NAME = GATEWAY.NAME
    GATEWAY_PORT = GATEWAY.PORT

    GATEWAYRESTPROXY = SERVICES.GATEWAYRESTPROXY
    GATEWAYRESTPROXY_NAME = GATEWAYRESTPROXY.NAME
    GATEWAYRESTPROXY_PORT = GATEWAYRESTPROXY.PORT

    ASSEMBLER = CONSTANTS.SERVICES.ASSEMBLER
    ASSEMBLER_NAME = ASSEMBLER.NAME
    ASSEMBLER_PORT = ASSEMBLER.PORT

    MANAGER = CONSTANTS.SERVICES.MANAGER
    MANAGER_NAME = MANAGER.NAME
    MANAGER_PORT = MANAGER.PORT

    PLATFORMS = CONSTANTS.PLATFORMS

    THREE = PLATFORMS.THREE
    RHINO = PLATFORMS.RHINO
    GRASSHOPPER = PLATFORMS.GRASSHOPPER

    GENERAL_FILEEXTENSIONS = CONSTANTS.GENERAL_FILEEXTENSIONS

    FILEEXTENSIONS_BYPLATFORMURL = {platform.URL:platform.FILEEXTENSION for platform in PLATFORMS.__dict__.values()}
    PLATFORM_BYFILEEXTENSION = {platform.FILEEXTENSION:Platform.Value('PLATFORM_'+platform.NAME.upper()) for platform in PLATFORMS.__dict__.values()}

    pass
    