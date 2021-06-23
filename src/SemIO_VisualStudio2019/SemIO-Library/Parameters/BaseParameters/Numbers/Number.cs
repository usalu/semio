namespace SemIOLibrary.Parameters.BaseParameters.Numbers
{
    public class Number : Parameter
    {
        public float Value { get; set; }

        public Number()
        {
        }

        public Number(float value)
        {
            Value = value;
        }
    }
}
