using System.Collections.Generic;
using SemIO.Parsing.ParserModels.Project.AbstractionLevels;

namespace SemIO.Parsing.ParserModels.Project
{
    /// <summary>
    /// The main model to store a project. In the moment only abstraction levels
    /// </summary>
    public class ProjectModel : InformedClass
    {
        public List<AbstractionLevelModel> AbstractionLevels { get; set; }

        public ProjectModel(string name, string description, string semIOCode = "") : base(name, description)
        {
            AbstractionLevels = Parser.ParseAbstractionLevels(semIOCode) ?? new List<AbstractionLevelModel>();
        }


        public ProjectModel(string name, string description) : base(name, description)
        {
           
        }

        public void AddAbstractionLevelsFromSemIOCode(string semIOCode)
        {
            AbstractionLevels.AddRange(Parser.ParseAbstractionLevels(semIOCode) ?? new List<AbstractionLevelModel>());
        }
    }
}
