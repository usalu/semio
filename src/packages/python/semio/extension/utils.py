from pydantic import Field

from gateway import ExtendingService
from utils import Server

class ExtensionServer (Server):
    gatway_address: str = "localhost:51000"
    extension: ExtendingService = Field(default_factory=ExtendingService)

