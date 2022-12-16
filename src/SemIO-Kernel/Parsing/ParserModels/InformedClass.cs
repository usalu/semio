namespace SemIO.Parsing.ParserModels
{
    /// <summary>
    /// Every class that can attributed later with a name and description
    /// </summary>
    public abstract class InformedClass
    {
        public string Name { get; }
        public string Description { get; }

        protected InformedClass(string name, string description)
        {
            Name = name;
            Description = description;
        }

    }
}
