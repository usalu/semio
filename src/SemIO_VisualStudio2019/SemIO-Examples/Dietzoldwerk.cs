using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SemIO.Parsing.ParserModels.Project;
using SemIO.Parsing.ParserModels.Project.AbstractionLevels;
using SemIO.Parsing.ParserModels.Project.AbstractionLevels.Things;

namespace SemIO
{
    public class Dietzoldwerk
    {
        public ProjectModel projectModel;

        public Dietzoldwerk()
        {
            projectModel = GenerateProjectModel();
        }

        public ProjectModel GenerateProjectModel() => new ProjectModel("Dietzoldwerk",
            "Extension for the Dietzoldwerk in Leipzig with artist ateliers, temporary living units and an event space.")
        {
            AbstractionLevels = new List<AbstractionLevelModel>()
            {
                new AbstractionLevelModel("Program", "Basis definitions and functional program",
                    new List<ParameterType>()
                    {
                        new ParameterType("Panels", "Available panels for the project to build on",
                            new string[]
                            {
                                "Rooftop",
                                "ExpansionFront",
                                "ExpansionSideWing"
                            }, true, new string[]
                            {
                                "",
                                "This part is an extension to the parcel of the neighbour.",
                                "This part consists of fragments of the ruins of the old boiler house."
                            })
                    },
                    new List<ThingModel>()
                    {
                        new ThingModel("ProjectDesign", "Main container for the design of the project",
                            new List<ThingParameter>()
                            {
                                new ThingParameter("Panels")
                            })
                    }
                )
            }
        };
    }
}
