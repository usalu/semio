namespace SemIO.Parsing.ParserModels
{
    /// <summary>
    /// A IParameter can be used for property and field member type generation later.
    /// </summary>
    public interface IParameter
    {
        string Name { get; }
        string Description { get; }
        string ParameterTypeName { get; }

    }
}
