using Grasshopper;
using Grasshopper.Kernel;
using Rhino.Geometry;
using System;
using System.Collections.Generic;
using Google.Protobuf.Collections;
using NetMQ;
using Semio.Gateway.V1;
using Semio.Model.V1;
using NetMQ.Sockets;
using Grasshopper.Kernel.Types;

namespace Semio.UI.Grasshopper.Components.Server
{
    public class LayoutDesignComponent : GH_Component
    {
        /// <summary>
        /// Each implementation of GH_Component must provide a public 
        /// constructor without any arguments.
        /// Category represents the Tab in which the component will appear, 
        /// Subcategory the panel. If you use non-existing tab or panel names, 
        /// new tabs/panels will automatically be created.
        /// </summary>
        public LayoutDesignComponent()
          : base("LayoutDesign", "LayoutDesign",
            "Description",
            "Semio", "Server")
        {
        }

        /// <summary>
        /// Registers all the input parameters for this component.
        /// </summary>
        protected override void RegisterInputParams(GH_InputParamManager pManager)
        {
            pManager.AddGenericParameter("Url", "Url", "Url of Semio server.", GH_ParamAccess.item);
            pManager.AddGenericParameter("Layout", "L", "Layout of design", GH_ParamAccess.item);
            pManager.AddBooleanParameter("Run", "R", "", GH_ParamAccess.item, false);
        }

        /// <summary>
        /// Registers all the output parameters for this component.
        /// </summary>
        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddGenericParameter("Design", "Design", "", GH_ParamAccess.item);
        }

        /// <summary>
        /// This is the method that actually does the work.
        /// </summary>
        /// <param name="DA">The DA object can be used to retrieve data from input parameters and 
        /// to store data in output parameters.</param>
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            string url = "";
            if (!DA.GetData(0, ref url)) return;
            if (url.StartsWith("https://")) url = url.Substring(8);
            else if (url.StartsWith("http://")) url = url.Substring(7);

            var obj = new GH_ObjectWrapper();
            if (!DA.GetData(1, ref obj)) return;
            if (obj == null) return;
            var layout = (Layout)obj.Value;

            bool run = false;
            if (!DA.GetData(2, ref run)) return;

            string response = "";
            using (var client = new RequestSocket("inproc://" + url))
            {
                var layoutDesignRequest = new LayoutDesignRequest()
                {
                    Layout = layout
                };

                var layoutDesignRequestString = layoutDesignRequest.ToString();
                client.SendFrame(layoutDesignRequestString);
                response = client.ReceiveFrameString();
            }

            DA.SetData(0, response);
        }

        /// <summary>
        /// Provides an Icon for every component that will be visible in the User Interface.
        /// Icons need to be 24x24 pixels.
        /// You can add image files to your project resources and access them like this:
        /// return Resources.IconForThisComponent;
        /// </summary>
        protected override System.Drawing.Bitmap Icon => null;

        /// <summary>
        /// Each component must have a unique Guid to identify it. 
        /// It is vital this Guid doesn't change otherwise old ghx files 
        /// that use the old ID will partially fail during loading.
        /// </summary>
        public override Guid ComponentGuid => new Guid("15ad0008-1e40-41ff-8e38-116102d7488a");
    }
}