from abc import ABC, abstractmethod

from .v1.translator_pb2 import Translating
from .v1.translator_pb2_grpc import TranslatorServiceServicer

from model import Pose,Representation
from extension.service import ExtensionService

class TranslatorService(ExtensionService,TranslatorServiceServicer,ABC):

    def getDescriptions(self) -> list[Translating]:
        return []

    @abstractmethod
    def translateRepresentation(self, representation:Representation, target_pose:Pose ,source_pose:Pose | None = None):
        pass
    
    def TranslateRepresentation(self, request, context):
        return self.translateRepresentation(request.representation,request.target_pose)