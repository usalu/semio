from semio.extension.translator.v1.translator_pb2 import Translating
from semio.extension.translator.v1.translator_pb2_grpc import TranslatorServiceServicer

from semio.model import Pose,Representation
from semio.extension.service import ExtensionService

class TranslatorService(ExtensionService,TranslatorServiceServicer):

    def _getDescriptions(self) -> list[Translating]:
        return []

    def translateRepresentation(self, representation:Representation, target_pose:Pose ,source_pose:Pose | None = None):
        raise NotImplementedError("This method needs to be overriden by the implementation if you want to use it.")

    def TranslateRepresentation(self, request, context):
        return self.translateRepresentation(request.representation,request.target_pose)