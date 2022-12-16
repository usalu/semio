namespace SemIO.Parsing.ParserModels.Project.AbstractionLevels
{
    /// <summary>
    /// Parameter types of an abstraction level that can be used as parameters for things inside the abstraction level.
    /// NOTE: This class only exists inside an abstraction level and can only be used inside there.
    /// </summary>
    public class ParameterType : InformedClass, IParameter
    {
        public string[] NameValues;
        public bool IsMultiset;
        public string[] ValueDescriptions;
        public int ValueCount() => NameValues.Length;

        public string ParameterTypeName => Name; // + "Type";

        public ParameterType(string name, string description, string[] nameValues, bool isMultiset,
            string[] valueDescriptions = null) : base(name, description)
        {
            NameValues = nameValues;
            IsMultiset = isMultiset;
            ValueDescriptions = valueDescriptions ?? new string[nameValues.Length];
        }
    }
}
