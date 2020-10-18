using System.Collections.Generic;

namespace SemIO.Parsing.ParserModels.Project.AbstractionLevels.Objects
{
    /// <summary>
    /// Object of an abstraction level that can be used as a parameter for an abstraction level inside the abstraction level.
    /// NOTE: This class only exists inside an abstraction level and can only be used inside there.
    /// </summary>
    public class ObjectModel : InformedClass, IParameter
    {
        public string ParameterTypeName => Name;// + "Model";
        public List<ObjectParameter> Parameters { get; }
        public List<string> ParentNames { get; }

        public ObjectModel(string name, string description, List<ObjectParameter> parameters,
            List<string> parentNames = null) : base(name, description)
        {
            if (parentNames != null)
                ParentNames = parentNames;
            else
                ParentNames = new List<string>(0);
            Parameters = parameters;
        }
    }
}
