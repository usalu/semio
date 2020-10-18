

namespace SemIO.Parsing.ParserModels.Project.AbstractionLevels.Objects
{
    /// <summary>
    /// Parameter for an object of an abstraction level containing a name, a type and a multiplicity
    /// </summary>
    public class ObjectParameter : InformedClass, IParameter
    {
        /*private string _parameterTypeName;
        public string ParameterTypeName
        {
            get => _parameterTypeName + "Type";
            set => _parameterTypeName = value.EndsWith("Type") ? value : value + "Type";
        }*/
        public string ParameterTypeName { get; }

        public Multiplicity Multiplicity { get; set; }

        public ObjectParameter(string name, string description, string parameterTypeName, Multiplicity multiplicity = null)
            : base(name, description)
        {
            ParameterTypeName = parameterTypeName;
            Multiplicity = multiplicity ?? new Multiplicity(1);
            
        }
    }
}
