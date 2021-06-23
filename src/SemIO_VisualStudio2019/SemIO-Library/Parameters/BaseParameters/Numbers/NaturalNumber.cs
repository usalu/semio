namespace SemIOLibrary.Parameters.BaseParameters.Numbers
{
    public class NaturalNumber : Number
    {
        public new uint Value { get; set; }

        public NaturalNumber()
        {
        }

        public NaturalNumber(uint value)
        {
            Value = value;
        }
    }
}
