// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;
using Google.Protobuf;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Model.V1;
using Encoding = System.Text.Encoding;

namespace Semio.UI.Grasshopper.Goos
{
    public class PlanGoo : SemioGoo<Plan>
    {
        public PlanGoo()
        {
            Value = new Plan();
        }

        public PlanGoo(Plan assembly)
        {
            Value = assembly;
        }

        public override IGH_Goo Duplicate() => new PlanGoo(Value.Clone());
        public override string TypeName => Plan.Descriptor.FullName;
        public override string TypeDescription => Plan.Descriptor.Declaration.LeadingComments;

        public override bool CastFrom(object source)
        {
            if (source == null) return false;

            switch (source)
            {
                case GH_String text:
                    try
                    {
                        Value = Plan.Parser.ParseJson(text.Value);
                        return true;
                    }
                    catch (Exception e)
                    {
                        try
                        {
                            Value = Plan.Parser.ParseFrom(ByteString.CopyFrom(text.Value, Encoding.Default));
                            return true;
                        }
                        catch (Exception exception)
                        {
                            try
                            {
                                // Check if string is a valid uri
                                //FileAttributes attr = File.GetAttributes(text.Value);
                                //bool isFile = (attr & FileAttributes.Normal) == FileAttributes.Normal;
                                //bool isUri = Uri.IsWellFormedUriString(text.Value, UriKind.RelativeOrAbsolute);
                                bool isUriCreated = Uri.TryCreate(text.Value, UriKind.Absolute, out Uri outUri);

                                if (!isUriCreated) return false;

                                Value = new Plan()
                                    {
                                        Uri = text.Value,
                                    };
                                return true;
                            }
                            catch (Exception e1)
                            {
                            }
                        }
                    }

                    return false;
                default:
                    return false;
            }
        }
    }
}