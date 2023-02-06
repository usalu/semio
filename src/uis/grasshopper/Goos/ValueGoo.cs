// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;
using Grasshopper.Documentation;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Utility;
using Point = Rhino.Geometry.Point;

namespace Semio.UI.Grasshopper.Goos
{
    public class ValueGoo : SemioGoo<Value>
    {
        public ValueGoo()
        {
            Value = new Value();
        }

        public ValueGoo(Value value)
        {
            Value = value;
        }
        public override IGH_Goo Duplicate() => new ValueGoo(Value.Clone());
        public override string TypeName => Value.Descriptor.FullName;
        public override string TypeDescription => Value.Descriptor.Declaration.LeadingComments;

        public override bool CastTo<Q>(ref Q target)
        {
            switch (target)
            {
                case GH_Integer: 
                    target = (Q)(object) new GH_Integer(Converter.ToInteger(Value));
                    return true;
                case GH_Number:
                    target = (Q)(object)new GH_Number(Converter.ToDouble(Value));
                    return true;
                case GH_String:
                    target = (Q)(object)new GH_String(Converter.ToString(Value));
                    return true;
            }


            if (typeof(Q).IsAssignableFrom(typeof(Point3d)))
            {
                object ptr = Converter.Convert(Value.Point);
                target = (Q)ptr;
                return true;
            }

            if (typeof(Q).IsAssignableFrom(typeof(GH_Number)))
            {
                double number = new();
                switch (Value.ValueCase)
                {
                    case Value.ValueOneofCase.NaturalNumber:
                        number = Value.NaturalNumber;
                        break;
                    case Value.ValueOneofCase.Number:
                        number = Value.Number;
                        break;
                    case Value.ValueOneofCase.IntegerNumber:
                        number = Value.IntegerNumber;
                        break;
                    case Value.ValueOneofCase.Text:
                        number = Convert.ToDouble(Value.Text);
                        break;
                }
                target = (Q)(object)new GH_Number(number);
                return true;
            }

            if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
            {
                object ptr = new GH_String(Value.Text);
                target = (Q)ptr;
                return true;
            }

            return false;
        }
        public override bool CastFrom(object source)
        {
            if (source == null) return false;

            switch (source)
            {
                case GH_Integer integer:
                    Value.IntegerNumber = integer.Value;
                    return true;
                case GH_Number number:
                    Value.Number = number.Value;
                    return true;
                case GH_String text:
                    Value.Text = text.Value;
                    return true;
            }

            Point3d point = new();
            if (GH_Convert.ToPoint3d(source, ref point, GH_Conversion.Both))
            {
                Value.Point = Converter.Convert(point);
                return true;
            }

            if (GH_Convert.ToDouble(source, out var numberConverted, GH_Conversion.Both))
            {
                Value.Number = numberConverted;
                return true;
            }

            if (GH_Convert.ToInt32(source, out var integerConverted, GH_Conversion.Both))
            {
                Value.IntegerNumber = integerConverted;
                return true;
            }

            if (GH_Convert.ToString(source, out var textConverted, GH_Conversion.Both))
            {
                Value.Text = textConverted;
                return true;
            }

            return false;
        }
    }
}