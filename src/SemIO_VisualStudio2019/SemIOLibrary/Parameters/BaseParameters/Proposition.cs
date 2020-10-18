namespace SemIOLibrary.Parameters.BaseParameters
{
    public class Proposition : Parameter
    {
        public bool Value { get; set; }

        public Proposition()
        {
        }

        public Proposition(bool value)
        {
            Value = value;
        }
    }
}
