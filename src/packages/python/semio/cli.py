from pydantic import BaseModel, Field
from constants import SERVICES
from argparse import ArgumentParser, Namespace

from constants import SERVICES_BYNAME

class ArgumentParserManager(BaseModel):
    argumentParser : ArgumentParser
    dependencies: list[str] = Field(default_factory=list)
    
    def addDependenciesArguments(self):
        for dependecy in self.dependencies:
            service = SERVICES_BYNAME[dependecy]
            self.argumentParser.add_argument(f'-{service.ABBREVIATION}a',f'--{service.NAME}address', const=service.NAME,
                                            help = f'The address of the assembler service without the port. \nExamples: localhost, 192.168.1.1, your.service.domain, ... \nDefault: {service.NAME}')
            