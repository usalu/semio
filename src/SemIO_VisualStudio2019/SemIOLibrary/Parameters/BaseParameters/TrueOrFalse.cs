namespace SemIOLibrary.Parameters.BaseParameters
{
    public class TrueOrFalse : Parameter
    {
        public bool Value { get; set; }

        public TrueOrFalse()
        {
        }

        public TrueOrFalse(bool value)
        {
            Value = value;
        }
    }
}
