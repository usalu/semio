from typing import Union
from rdflib import Graph
from .artifact import Artifact

class Compiler:

    
    async def synthesize(self, synthesis:Union[Artifact,Graph,str])->Graph:
        """Compiles a design."""
        if not isinstance(synthesis,Artifact):
            synthesis = await Artifact.Parse("Synthesis",synthesis)

        if synthesis.formationPlan:
            synthesis.layout =  self.computeScript(self.reducePlan((synthesis.formationPlan)))

        if synthesis.scheme:
            assert not synthesis.arrangement, "There can't be an arrangement and a plan for a scheme because the scheme produces an arrangement. If you want to use the generated arrangement then delete the arrangement, and vice versa."
            synthesis.arrangement = (self.computeScript(self.reducePlan((synthesis.schemePlan))))

        if synthesis.arrangement:


        # await self.generate(model)
        # await self.prepare()

        design = None
        return design

    """Generate all genes from generations, add the genes to the model and remove the generations from the model."""
    async def generate(self, model, generation):
        gene = Generation(self.computeScript(generation))
        model.add(gene)
        model.remove(generation)
        assert not model.generation
        return model

    """."""
    async def prepare(self, model):
        pass

     """."""
    async def reducePlan(self, model, plan):
        for generation in plan.generations:
            self.generate(model, generation)
        
    
    """Computes a script with appropriate adaper."""
    async def computeScript(self, plan):
        return await self.adapters[hash(plan.environment)].adapt(plan)
    
